use winit::event::*;

use crate::device::container::Container;
use crate::device::display_window::WGContext;
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::RenderUtil;
use crate::widget::component::ComponentModel;
use crate::widget::listener::Listener;

/// 默认窗口帧背景色
const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

/// 窗口帧结构体
/// 作用：用作gui控件的容器
pub struct Frame {
    pub glob_pipeline: PipelineState,
    pub comp_graph_arr: Vec<Box<dyn ComponentModel>>,
    pub wgcontext: WGContext,
}

impl Frame {
    fn new(wgcontext: WGContext) -> Self {
        let glob_pipeline = PipelineState::default(&wgcontext.device);

        let comp_graph_arr = Vec::with_capacity(20);
        Self {
            glob_pipeline,
            comp_graph_arr,
            wgcontext,
        }
    }

    fn add_comp_arr(&mut self, mut comp: Box<dyn ComponentModel>) {
        self.comp_graph_arr.push(comp);
    }
}

impl Container for Frame {
    fn new(wgcontext: WGContext) -> Self {
        Frame::new(wgcontext)
    }

    fn add_comp<C>(&mut self, comp: C)
        where C: ComponentModel + Listener + 'static {
        self.add_comp_arr(Box::new(comp))
    }

    /*  fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
          // self.size = new_size;
          // self.sc_desc.width = new_size.width;
          // self.sc_desc.height = new_size.height;
          // self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
      }*/

    fn input(&mut self, event: &WindowEvent) -> bool {
        // for listener in &self.graph_context {}
        let mut input = false;
        for comp in &mut self.comp_graph_arr {
            input = comp.key_listener(event);
        }
        input
    }

    fn render(&mut self) {
        let screen_displayed = self.wgcontext
            .device
            .create_swap_chain(&self.wgcontext.surface, &self.wgcontext.sc_desc);

        let target = screen_displayed.get_current_frame().unwrap().output;
        let mut encoder = self.wgcontext.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &target.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }
        let mut utils = RenderUtil {
            encoder,
            target,
        };
        log::info!("graph_context size:{}", self.comp_graph_arr.len());
        for view in &mut self.comp_graph_arr {
            view.draw(&self.wgcontext, &mut utils, &self.glob_pipeline);
        }
        self.wgcontext.queue.submit(std::iter::once(utils.encoder.finish()));
    }
}

