use std::fmt::Debug;

use winit::window::Window;

use crate::backend::wgpu_impl::*;
use crate::device::Container;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;

/// 图形渲染上下文结构体
/// 作用：封装wgpu渲染所需的结构体
#[derive(Debug)]
pub struct GPUContext {
    /// 渲染面板
    pub surface: wgpu::Surface,
    /// 图形设备
    pub device: wgpu::Device,
    /// 渲染命令队列
    pub queue: wgpu::Queue,
    /// 交换缓冲区描述符
    sc_desc: wgpu::SurfaceConfiguration,
    /// 渲染管道
    pub glob_pipeline: PipelineState,
}

impl GPUContext {
    pub async fn new(window: &Window) -> GPUContext {
        log::info!("Initializing the surface...");
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let size = window.inner_size();

        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Request adapter");

        let format = surface
            .get_preferred_format(&adapter)
            .expect("Get preferred format");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
                },
                None, // Trace path
            )
            .await
            .unwrap();
        //  : wgpu::TextureFormat::Bgra8UnormSrgb
        let sc_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let glob_pipeline = PipelineState::default(&device);

        surface.configure(&device, &sc_desc);
        GPUContext {
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

    /// 获取当前帧尺寸
    pub fn get_surface_size(&self) -> Point<u32> {
        Point::new(self.sc_desc.width, self.sc_desc.height)
    }

    /// 显示图形内容
    pub fn present<C, M>(&mut self,
                         container: &mut C)
        where C: Container<M> + 'static, M: 'static + Debug
    {
        match self.surface.get_current_texture() {
            Err(error) => {
                log::error!("{}", error);
            }
            Ok(target_view) => {
                let mut utils
                    = RenderUtil::new(&target_view, self);
                utils.clear_frame(BACKGROUND_COLOR);
                container.render(&mut utils);
                utils.context.queue.submit(Some(utils.encoder.finish()));
                target_view.present();
            }
        }
    }
}