use winit::event::*;

use crate::device::Container;
use crate::device::ELContext;
use crate::device::WGContext;
use crate::graphic::base::*;
use crate::graphic::render_middle::PipelineState;
use crate::graphic::render_middle::RenderUtil;
use crate::widget::{component, Instance, Panel};
use crate::widget::component::ComponentModel;

/// 窗口帧结构体
/// 作用：用作gui控件的容器
pub struct Frame<M: std::cmp::PartialEq + std::marker::Copy> {
    pub glob_pipeline: PipelineState,
    pub display_panel: Option<Panel<M>>,
    // pub wgcontext: WGContext,
}

impl<M: Copy + PartialEq> Frame<M> {
    fn new(wgcontext: &WGContext) -> Self {
        let glob_pipeline = PipelineState::default(&wgcontext.device);

        // let comp_graph_arr = Vec::new();
        Self {
            glob_pipeline,
            display_panel: None,
            // wgcontext,
        }
    }

    pub fn add_widgets(&mut self, widgets: Panel<M>) {
        self.display_panel = Some(widgets);
    }
}

impl<M: Copy + PartialEq> Container<M> for Frame<M> {
    fn new(wgcontext: &WGContext) -> Self {
        Frame::new(wgcontext)
    }

    fn add_comp(&mut self, instance: &impl Instance<M=M>) {
        // let mut arr = Vec::new();
        // arr.push(comp);
        self.add_widgets(instance.layout())
    }

    fn input(&mut self, wgcontext: &mut WGContext, el_context: &mut ELContext<'_, M>, instance: &mut impl Instance<M=M>) -> bool
    {
        match el_context.window_event.as_ref().unwrap() {
            WindowEvent::Resized(new_size) => {
                // 更新swapChain交换缓冲区
                wgcontext
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
        for panel in &mut self.display_panel {
            for comp in &mut panel.widgets {
                if component::component_listener::<M>(comp, el_context) {
                    input = true;
                }
            }
        }
        if el_context.message.is_some() {
            instance.update(el_context.message.as_ref().unwrap());
            self.add_widgets(instance.layout());
            // 清除消息，防止重复发送
            el_context.message = None;
        }
        input
    }

    fn render(&mut self, wgcontext: &mut WGContext) {
        match wgcontext.surface.get_current_texture() {
            Err(error) => {
                log::error!("{}", error);
            }
            Ok(frame) => {
                let mut utils
                    = RenderUtil::new(&frame, wgcontext, &self.glob_pipeline);
                utils.clear_frame(BACKGROUND_COLOR);
                for comp in &self.display_panel {
                    comp.draw(&mut utils)
                }
                utils.context.queue.submit(Some(utils.encoder.finish()));
                frame.present();
            }
        }
    }
}

