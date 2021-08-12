use winit::window::Window;

/// 图形渲染上下文结构体
/// 作用：封装wgpu渲染所需的结构体
pub struct WGContext {
    /// 渲染面板
    pub surface: wgpu::Surface,
    /// 图形设备
    pub device: wgpu::Device,
    /// 渲染命令队列
    pub queue: wgpu::Queue,
    /// 交换缓冲区描述符
    pub sc_desc: wgpu::SwapChainDescriptor,
}

impl WGContext {
    pub async fn new(window: &Window) -> WGContext {
        log::info!("Initializing the surface...");
        let instance = wgpu::Instance::new(wgpu::BackendBit::DX11);
        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(window);
            (size, surface)
        };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

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
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        WGContext {
            surface,
            device,
            queue,
            sc_desc,
        }
    }
}