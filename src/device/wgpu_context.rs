use winit::window::Window;

use crate::graphic::base::shape::Point;

/// 图形渲染上下文结构体
/// 作用：封装wgpu渲染所需的结构体
#[derive(Debug)]
pub struct WGContext {
    /// 渲染面板
    pub surface: wgpu::Surface,
    /// 图形设备
    pub device: wgpu::Device,
    /// 渲染命令队列
    pub queue: wgpu::Queue,
    /// 交换缓冲区描述符
    sc_desc: wgpu::SurfaceConfiguration,
}

impl WGContext {
    pub async fn new(window: &Window) -> WGContext {
        log::info!("Initializing the surface...");
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let size = window.inner_size();

        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
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
                    limits: wgpu::Limits::default(),
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
        surface.configure(&device, &sc_desc);

        WGContext {
            surface,
            device,
            queue,
            sc_desc,
        }
    }
    // 更新交换缓冲区
    pub fn update_surface_configure(&mut self, size: Point<u32>) {
        self.sc_desc.width = size.x;
        self.sc_desc.height = size.y;
        self.surface.configure(&self.device, &self.sc_desc);
    }

    pub fn get_surface_size(&self) -> Point<u32> {
        Point::new(self.sc_desc.width, self.sc_desc.height)
    }
}