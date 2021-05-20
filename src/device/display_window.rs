use std::any::Any;
use std::future::Future;

use futures::{StreamExt, task};
use futures::channel::mpsc;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::device::painter::Painter;
use crate::graphic::base::rectangle::Rectangle;

pub struct DisplayWindow {
    pub window: winit::window::Window,
    pub event_loop: EventLoop<()>,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub wgcontext: WGContext,
}

pub struct WGContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
}

struct Application {
    base: Rectangle,
}

impl DisplayWindow {
    /// 初始化窗口
    pub async fn init<E: Painter>(mut builder: WindowBuilder) -> DisplayWindow {
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
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        DisplayWindow {
            window,
            event_loop,
            size,
            wgcontext: WGContext {
                surface,
                device,
                queue,
                sc_desc,
                swap_chain,
            },
        }
    }

    pub fn start<E>(event_loop: EventLoop<()>, mut container: E)
        where E: Painter + 'static
    {
        log::info!("Entering render loop...");
        let (mut sender, receiver) = mpsc::unbounded();
        let mut instance = Box::pin(event_listener(container, receiver));
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
}

pub async fn event_listener<T, E>(mut container: E, mut receiver: mpsc::UnboundedReceiver<winit::event::Event<'_, T>>)
    where T: std::fmt::Debug, E: Painter + 'static
{
    while let Some(event) = receiver.next().await {
        // log::info!("{:#?}", event);
        match event {
            // Event::WindowEvent {
            //     ref event,
            //     window_id,
            // } if window_id == display_device.window.id() => {
            //     if !container.input(event) {
            //         match event {
            //             WindowEvent::Resized(physical_size) => {
            //                 // state.resize(*physical_size);
            //                 display_device.size = *physical_size;
            //                 display_device.sc_desc.width = physical_size.width;
            //                 display_device.sc_desc.height = physical_size.height;
            //                 display_device.swap_chain = display_device.device.create_swap_chain(&display_device.surface, &display_device.sc_desc);
            //             }
            //             _ => {}
            //         }
            //     }
            // }
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