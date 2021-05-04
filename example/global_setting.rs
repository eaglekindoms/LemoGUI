use std::iter;

use wgpu::RenderPipeline;
use winit::event::*;

use LemoGUI::device::display_window::DisplayWindow;
use LemoGUI::device::painter::Painter;
use LemoGUI::graphic::render_type::pipeline_state::PipelineState;
use LemoGUI::graphic::render_type::render_function::{Render, RenderGraph};
use LemoGUI::graphic::shape::point::Rectangle;
use LemoGUI::model::button::*;

const INDICES: &[u16] = &[0, 2, 1, 3];
const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

pub struct GlobalState {
    pub glob_pipeline: PipelineState,
    pub button: RenderGraph,
}

impl Painter for GlobalState {
    fn new(sc_desc: &wgpu::SwapChainDescriptor,
           device: &wgpu::Device,
           queue: &wgpu::Queue, ) -> Self {
        let glob_pipeline = PipelineState::create_glob_pipeline(device);
        let rect = Rectangle::new(100.0, 100.0, 400, 40);

        // 自定义设置
        let button = Button::default(&rect, "button1").to_graph(device, sc_desc, queue);

        Self {
            glob_pipeline,
            button,
        }
    }

    /*  fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
          // self.size = new_size;
          // self.sc_desc.width = new_size.width;
          // self.sc_desc.height = new_size.height;
          // self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
      }*/

    fn input(&mut self, event: &WindowEvent) -> bool {
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
                    // self.use_complex = 1;
                } else if *state == ElementState::Released {
                    // self.use_complex = 0;
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
                    // self.use_complex = 2;
                } else if *state == ElementState::Released {
                    // self.use_complex = 0;
                }
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {}

    fn render(&mut self,
              encoder: &mut wgpu::CommandEncoder,
              target: &wgpu::TextureView) -> Result<(), wgpu::SwapChainError> {
        log::info!("render a frame");
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        self.button.render(&mut render_pass, &self.glob_pipeline, false);

        Ok(())
    }
}

