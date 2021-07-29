use std::future::Future;

use futures::{StreamExt, task};
use futures::channel::mpsc;
use winit::event::*;
use winit::event_loop::*;
use winit::window::*;

use crate::device::container::Container;

/// 窗口结构体
/// 作用：封装窗体，事件循环器，图形上下文
pub struct DisplayWindow {
    /// 窗体
    pub window: Window,
    /// 事件监听器
    pub event_loop: EventLoop<()>,
    /// 窗体尺寸
    pub size: winit::dpi::PhysicalSize<u32>,
    /// 图形上下文
    pub wgcontext: WGContext,
}

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

impl DisplayWindow {
    /// 初始化窗口
    pub async fn init(builder: WindowBuilder) -> DisplayWindow {
        log::info!("Initializing the window...");
        let event_loop = winit::event_loop::EventLoop::new();
        let window = builder.build(&event_loop).unwrap();

        log::info!("Initializing the surface...");
        let instance = wgpu::Instance::new(wgpu::BackendBit::DX11);
        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(&window);
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

        DisplayWindow {
            window,
            event_loop,
            size,
            wgcontext: WGContext {
                surface,
                device,
                queue,
                sc_desc,
            },
        }
    }

    /// 启动窗口事件循环器
    pub fn start<C>(window: Window, event_loop: EventLoop<()>, container: C)
        where C: Container + 'static
    {
        log::info!("Entering render loop...");
        let (mut sender, receiver) = mpsc::unbounded();
        let mut instance = Box::pin(event_listener(window, container, receiver));
        let mut context = task::Context::from_waker(task::noop_waker_ref());
        event_loop.run(move |event, _, control_flow| {
            if let ControlFlow::Exit = control_flow {
                return;
            }
            let event = match event {
                Event::WindowEvent {
                    event:
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size,
                        ..
                    },
                    window_id,
                } => Some(Event::WindowEvent {
                    event: WindowEvent::Resized(*new_inner_size),
                    window_id,
                }),
                _ => event.to_static(),
            };
            if let Some(event) = event {
                sender.start_send(event).expect("Send event");
                let poll = instance.as_mut().poll(&mut context);
                *control_flow = match poll {
                    task::Poll::Pending => {
                        // log::info!("pending");
                        ControlFlow::Wait
                    }
                    task::Poll::Ready(_) => {
                        // log::info!("--------ready--------");
                        ControlFlow::Exit
                    }
                };
            }
        });
    }

    /// 装填组件容器，启动窗口
    pub fn start_window<C>(builder: WindowBuilder, build_container: &Fn(WGContext) -> C)
        where C: Container + 'static
    {
        use futures::executor::block_on;
        let display_device = block_on(DisplayWindow::init(builder));
        log::info!("Initializing the example...");
        DisplayWindow::start::<C>(display_device.window, display_device.event_loop,
                                  build_container(display_device.wgcontext));
    }
}

/// 事件监听方法
pub async fn event_listener<T, C>(window: Window, mut container: C, mut receiver: mpsc::UnboundedReceiver<winit::event::Event<'_, T>>)
    where T: std::fmt::Debug, C: Container + 'static
{
    while let Some(event) = receiver.next().await {
        // log::info!("{:#?}", event);
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if container.input(event) {
                    container.render();
                }
            }
            Event::RedrawRequested(_) => {
                // state.update();
                container.render();
            }
            // Event::MainEventsCleared => {
            //     // RedrawRequested will only trigger once, unless we manually
            //     // request it.
            //     display_device.window.request_redraw();
            // }
            _ => {}
        }
    };
}