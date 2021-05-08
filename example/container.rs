use std::iter;

use winit::event::*;

use LemoGUI::device::display_window::DisplayWindow;
use LemoGUI::device::listener::Listener;
use LemoGUI::device::painter::Painter;
use LemoGUI::graphic::render_type::pipeline_state::PipelineState;
use LemoGUI::graphic::render_type::render_function::{Render, RenderGraph};
use LemoGUI::graphic::shape::rectangle::Rectangle;
use LemoGUI::model::component::{Component, ComponentModel};

const INDICES: &[u16] = &[0, 2, 1, 3];
const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

pub struct GlobalState {
    pub glob_pipeline: PipelineState,
    pub graph_context: Vec<Box<dyn ComponentModel>>,
}

impl Painter for GlobalState {
    fn new(display_device: &DisplayWindow) -> Self {
        let glob_pipeline = PipelineState::create_glob_pipeline(&display_device.device);
        let graph_context = Vec::with_capacity(20);
        Self {
            glob_pipeline,
            graph_context,
        }
    }

    fn add_comp<C: ComponentModel>(&mut self, display_device: &DisplayWindow, comp: C) {
        // self.graph_context.push(comp.to_graph(display_device));
    }

    /*  fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
          // self.size = new_size;
          // self.sc_desc.width = new_size.width;
          // self.sc_desc.height = new_size.height;
          // self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
      }*/

    fn input(&mut self, event: &WindowEvent) -> bool {
        // for listener in &self.graph_context {}
        false
    }

    fn update(&mut self) {}

    fn render(&mut self, display_window: &DisplayWindow,
              encoder: &mut wgpu::CommandEncoder,
              target: &wgpu::TextureView) -> Result<(), wgpu::SwapChainError> {
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

        // for view in &self.graph_context {
        //     view.render(&mut render_pass, &self.glob_pipeline, false);
        // }

        Ok(())
    }
}

