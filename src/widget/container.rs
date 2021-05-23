use winit::event::*;

use crate::device::display_window::WGContext;
use crate::device::painter::Painter;
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::RenderGraph;
use crate::widget::component::{Component, ComponentModel};
use crate::widget::listener::Listener;

const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

pub struct Container {
    pub glob_pipeline: PipelineState,
    pub comp_graph_arr: Vec<RenderGraph>,
    // pub comp_data_arr: Vec<Box<Component>>,
    pub wgcontext: WGContext,
}

impl Container {
    fn new(wgcontext: WGContext) -> Self {
        let glob_pipeline = PipelineState::create_glob_pipeline(&wgcontext.device);
        let comp_graph_arr = Vec::with_capacity(20);
        // let comp_data_arr = Vec::with_capacity(20);
        Self {
            glob_pipeline,
            comp_graph_arr,
            // comp_data_arr,
            wgcontext,
        }
    }
    fn update_comp_arr<C>(&mut self, comp: &mut C)
        where C: ComponentModel {
        if self.comp_graph_arr.len() == 0 {
            log::info!("push the first component");
            self.comp_graph_arr.push(comp.to_graph(&self.wgcontext));
            comp.set_index(self.comp_graph_arr.len() - 1);
            // self.comp_data_arr.insert(comp.get_index().unwrap(), Box::new(comp));
        } else if self.comp_graph_arr.len() != 0 {
            log::info!("-----update component array-----");
            log::info!("get current componet index: {:#?}", comp.get_index());
            if comp.get_index() != None {
                self.comp_graph_arr
                    .insert(comp.get_index().unwrap(), comp.to_graph(&self.wgcontext));
                // self.comp_data_arr.insert(comp.get_index().unwrap(), Box::new(comp));
            } else {
                self.comp_graph_arr
                    .push(comp.to_graph(&self.wgcontext));
                comp.set_index(self.comp_graph_arr.len() - 1);
                // self.comp_data_arr.insert(comp.get_index().unwrap(), Box::new(comp));
            }
        }
    }
}

impl Painter for Container {
    fn new(wgcontext: WGContext) -> Self {
        Container::new(wgcontext)
    }

    fn add_comp<C>(&mut self, comp: &mut C)
        where C: ComponentModel + Listener {
        self.update_comp_arr(comp)
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
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            log::info!("graph_context size:{}", self.comp_graph_arr.len());
            for view in &self.comp_graph_arr {
                view.draw_rect(&mut render_pass, &self.glob_pipeline, false);
            }
        }

        self.wgcontext.queue.submit(std::iter::once(encoder.finish()));
    }
}

