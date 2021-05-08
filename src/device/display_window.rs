use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::device::painter::Painter;

pub struct DisplayWindow {
    pub window: winit::window::Window,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
}

impl DisplayWindow {
    /// 初始化窗口
    pub async fn init<E: Painter>(mut builder: WindowBuilder, event_loop: &EventLoop<()>) -> DisplayWindow {
        log::info!("Initializing the window...");
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
            size,
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
        }
    }

    pub fn start<E>
    (mut self, mut container: E, event_loop: EventLoop<()>)
        where E: Painter + 'static
    {
        log::info!("Entering render loop...");
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    if !container.input(event) {
                        match event {
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            WindowEvent::KeyboardInput { input, .. } => match input {
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                _ => {}
                            },
                            WindowEvent::Resized(physical_size) => {
                                // state.resize(*physical_size);
                                self.size = *physical_size;
                                self.sc_desc.width = physical_size.width;
                                self.sc_desc.height = physical_size.height;
                                self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
                            }
                            WindowEvent::ScaleFactorChanged
                            {
                                new_inner_size, ..
                            } => {
                                // new_inner_size is &mut so w have to dereference it twice
                                // state.resize(**new_inner_size);
                                self.size = **new_inner_size;
                                self.sc_desc.width = new_inner_size.width;
                                self.sc_desc.height = new_inner_size.height;
                                self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(_) => {
                    // state.update();
                    let frame = self.swap_chain.get_current_frame().unwrap().output;
                    let mut encoder = self.device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });
                    match container.render(&self, &mut encoder, &frame.view) {
                        Ok(_) => {}
                        // Recreate the swap_chain if lost
                        Err(wgpu::SwapChainError::Lost) => {}
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    };
                    self.queue.submit(std::iter::once(encoder.finish()));
                }
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}