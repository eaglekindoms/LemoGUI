use std::fmt::Debug;

use wgpu::{Instance, RenderPipeline};

use crate::backend::wgpu_impl::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::widget::ComponentModel;

/// 图形渲染上下文结构体
/// 作用：封装wgpu渲染所需的结构体
#[derive(Debug)]
pub struct WGPUContext {
    /// 渲染面板
    surface: wgpu::Surface<'static>,
    /// 图形设备
    pub device: wgpu::Device,
    /// 渲染命令队列
    pub queue: wgpu::Queue,
    /// 交换缓冲区描述符
    sc_desc: wgpu::SurfaceConfiguration,
    /// 渲染管道
    glob_pipeline: PipelineState,
}

impl WGPUContext {
    /// 创建 wgpu Instance
    pub fn create_instance() -> Instance {
        Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: Default::default(),
            display: None,
        })
    }

    /// 从已创建的 surface 初始化渲染上下文
    pub async fn new(
        surface: wgpu::Surface<'static>,
        instance: &Instance,
        window_size: Point<u32>,
    ) -> WGPUContext {
        log::info!("Initializing the surface...");
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Request adapter");

        let caps = surface.get_capabilities(&adapter);
        let present_modes = caps.present_modes;
        let alpha_modes = caps.alpha_modes;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::default(),
                experimental_features: Default::default(),
                trace: Default::default(),
            })
            .await
            .unwrap();

        let sc_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window_size.x,
            height: window_size.y,
            present_mode: present_modes[0],
            alpha_mode: alpha_modes[0],
            view_formats: vec![wgpu::TextureFormat::Bgra8UnormSrgb],
            desired_maximum_frame_latency: 2,
        };
        let glob_pipeline = PipelineState::default(&device);

        surface.configure(&device, &sc_desc);
        WGPUContext {
            surface,
            device,
            queue,
            sc_desc,
            glob_pipeline,
        }
    }
    // 更新交换缓冲区
    pub fn update_surface_configure<P: Into<Point<u32>>>(&mut self, size: P) {
        let size = size.into();
        self.sc_desc.width = size.x;
        self.sc_desc.height = size.y;
        self.surface.configure(&self.device, &self.sc_desc);
    }
    /// 获取渲染管线
    pub fn get_pipeline(&self, shape_type: ShapeType) -> Option<&RenderPipeline> {
        self.glob_pipeline.get_pipeline(shape_type)
    }
    /// 获取当前帧尺寸
    pub fn get_surface_size(&self) -> Point<u32> {
        Point::new(self.sc_desc.width, self.sc_desc.height)
    }

    /// 显示图形内容
    pub fn present<C, M>(&mut self, container: &mut C, font_map: &mut GCharMap)
    where
        C: ComponentModel<M> + 'static,
        M: 'static + Debug,
    {
        let surface_texture = self.surface.get_current_texture();
        let target_view = match surface_texture {
            wgpu::CurrentSurfaceTexture::Success(tex)
            | wgpu::CurrentSurfaceTexture::Suboptimal(tex) => tex,
            other => {
                log::error!("get_current_texture failed: {:?}", other);
                return;
            }
        };
        let mut utils = RenderUtil::new(&target_view, self);
        utils.clear_frame(BACKGROUND_COLOR);
        container.draw(&mut utils, font_map);
        let _submission = utils.context.queue.submit(Some(utils.encoder.finish()));
        target_view.present();
    }
}
