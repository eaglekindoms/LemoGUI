use winit::event::*;

use crate::device::container::Container;
use crate::device::event_context::ELContext;
use crate::device::wgpu_context::WGContext;
use crate::graphic::base::shape::Point;
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::RenderUtil;
use crate::widget::component::ComponentModel;
use crate::widget::listener;

/// 默认窗口帧背景色
const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

/// 窗口帧结构体
/// 作用：用作gui控件的容器
pub struct Frame<M> {
    pub glob_pipeline: PipelineState,
    pub comp_graph_arr: Vec<Box<dyn ComponentModel<M>>>,
    pub wgcontext: WGContext,
}

impl<M> Frame<M> {
    fn new(wgcontext: WGContext) -> Self {
        let glob_pipeline = PipelineState::default(&wgcontext.device);

        let comp_graph_arr = Vec::with_capacity(20);
        Self {
            glob_pipeline,
            comp_graph_arr,
            wgcontext,
        }
    }

    fn add_comp_arr(&mut self, comp: Box<dyn ComponentModel<M>>) {
        self.comp_graph_arr.push(comp);
    }
}

impl<M> Container<M> for Frame<M> {
    fn new(wgcontext: WGContext) -> Self {
        Frame::new(wgcontext)
    }

    fn add_comp<C>(&mut self, comp: C)
        where C: ComponentModel<M> + 'static {
        self.add_comp_arr(Box::new(comp))
    }

    fn input(&mut self, el_context: &mut ELContext<'_, M>) -> bool
    {
        match el_context.window_event.as_ref().unwrap() {
            WindowEvent::Resized(new_size) => {
                // 更新swapChain交换缓冲区
                self.wgcontext
                    .update_surface_configure(Point::new(new_size.width, new_size.height));
            }
            // 储存鼠标位置坐标
            WindowEvent::CursorMoved { position, .. }
            => {
                el_context.update_cursor(Point::new(position.x as f32, position.y as f32));
            }
            _ => {}
        }
        let mut input = false;
        for comp in &mut self.comp_graph_arr {
            if listener::component_listener::<M>(comp, el_context) {
                input = true;
            }
            // 清除消息，防止重复发送
            el_context.message = None;
        }
        input
    }

    fn render(&mut self) {
        match self.wgcontext.surface.get_current_frame() {
            Err(error) => {
                log::error!("{}", error);
            }
            Ok(frame) => {
                let view = frame.output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = self.wgcontext.device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });
                {
                    let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
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
                    view,
                };
                log::info!("graph_context size:{}", self.comp_graph_arr.len());
                for comp in &mut self.comp_graph_arr {
                    comp.draw(&self.wgcontext, &mut utils, &self.glob_pipeline);
                }
                self.wgcontext.queue.submit(Some(utils.encoder.finish()));
            }
        }
    }
}

