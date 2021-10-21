use crate::device::Container;
use crate::device::ELContext;
use crate::graphic::render_middle::RenderUtil;
use crate::widget::{component, Instance, Panel};
use crate::widget::component::ComponentModel;

/// 窗口帧结构体
/// 作用：用作gui控件的容器
pub struct Frame<M: std::cmp::PartialEq + std::marker::Copy> {
    pub display_panel: Vec<Panel<M>>,
}

impl<M: Copy + PartialEq> Frame<M> {
    pub fn new() -> Self {
        Self {
            display_panel: Vec::with_capacity(2),
        }
    }

    pub fn add_widgets(&mut self, widgets: Panel<M>) {
        self.display_panel.push(widgets)
    }
}

impl<M: Copy + PartialEq> Container<M> for Frame<M> {
    fn input(&mut self, el_context: &mut ELContext<'_, M>, instance: &mut impl Instance<M=M>) -> bool
    {
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

    fn render(&self, utils: &mut RenderUtil) {
        for comp in &self.display_panel {
            comp.draw(utils)
        }
    }
}

