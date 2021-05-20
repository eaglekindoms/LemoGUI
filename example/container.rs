use std::iter;

use winit::event::*;

use LemoGUI::device::display_window::{DisplayWindow, WGContext};
use LemoGUI::device::listener::Listener;
use LemoGUI::device::painter::Painter;
use LemoGUI::graphic::render_type::pipeline_state::PipelineState;
use LemoGUI::graphic::render_type::render_function::RenderGraph;
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
    pub comp_grap_stack: Vec<RenderGraph>,
    pub wgcontext: WGContext,
}

impl GlobalState {
    fn new(wgcontext: WGContext) -> Self {
        let glob_pipeline = PipelineState::create_glob_pipeline(&wgcontext.device);
        let comp_grap_stack = Vec::with_capacity(20);

        Self {
            glob_pipeline,
            comp_grap_stack,
            wgcontext,
        }
    }
}

impl Painter for GlobalState {
    fn new(wgcontext: WGContext) -> Self {
        GlobalState::new(wgcontext)
    }

    fn add_comp<C>(&mut self, comp: &mut C)
        where C: ComponentModel + Listener {
        if self.comp_grap_stack.len() == 0 {
            log::info!("push the first component");
            self.comp_grap_stack.push(comp.to_graph(&self.wgcontext));
            comp.set_index(self.comp_grap_stack.len() - 1);
        } else if self.comp_grap_stack.len() != 0 {
            log::info!("-----update component array-----");
            log::info!("get current componet index: {:#?}", comp.get_index());
            if comp.get_index() != None {
                self.comp_grap_stack
                    .insert(comp.get_index().unwrap(), comp.to_graph(&self.wgcontext));
            } else {
                self.comp_grap_stack
                    .push(comp.to_graph(&self.wgcontext));
                comp.set_index(self.comp_grap_stack.len() - 1);
            }
        }
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

    fn render(&mut self) {
        let frame = self.wgcontext.swap_chain.get_current_frame().unwrap().output;
        let mut encoder = self.wgcontext.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
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

            log::info!("graph_context size:{}", self.comp_grap_stack.len());
            for view in &self.comp_grap_stack {
                view.draw_rect(&mut render_pass, &self.glob_pipeline, false);
            }
        }

        self.wgcontext.queue.submit(std::iter::once(encoder.finish()));
    }
}

