use std::iter;

const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use wgpu::*;
use crate::backend::shape::*;
use crate::backend::shader::Shader;
use crate::backend::buffer_state::*;
use crate::backend::render::*;
use crate::backend::pipeline_state::PipelineState;

use crate::widget::button::*;

const INDICES: &[u16] = &[0, 2, 1, 3];

pub struct GlobalState {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    use_complex: u8,
}

impl GlobalState {
    pub async fn new(window: &Window) -> Self {
        // ---
        let size = window.inner_size();
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::DX11);
        let surface = unsafe { instance.create_surface(window) };
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
        let use_complex = 0;
        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            use_complex,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
                ..
            } => {
                if *state == ElementState::Pressed {
                    self.use_complex = 1;
                } else if *state == ElementState::Released {
                    self.use_complex = 0;
                }
                true
            }
            WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::LAlt),
                    ..
                },
                ..
            } => {
                if *state == ElementState::Pressed {
                    self.use_complex = 2;
                } else if *state == ElementState::Released {
                    self.use_complex = 0;
                }
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self, rect: &Rectangle) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let glob_pipeline = PipelineState::create_glob_pipeline(&self);

        // 自定义设置
        let button = Button::default(rect, "button1").to_graph(&self);
        let button1 = Button::default(&Rectangle::new(20.0, 200.0, 300, 42), "button2").to_graph(&self);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        button.render(&mut render_pass, &glob_pipeline, self.use_complex == 1);
        button1.render(&mut render_pass, &glob_pipeline, self.use_complex == 2);
        drop(render_pass);
        self.queue.submit(iter::once(encoder.finish()));

        Ok(())
    }

    pub fn render_a_button() {}
}


