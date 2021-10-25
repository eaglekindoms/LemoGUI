use crate::device::Container;
use crate::device::ELContext;
use crate::graphic::render_middle::RenderUtil;
use crate::widget::{Instance, Panel};
use crate::widget::component::ComponentModel;

/// 窗口帧结构体
/// 作用：用作gui控件的容器
pub struct Frame<M: PartialEq + Copy, I: Instance<M=M>> {
    pub display_panel: Vec<(I, Panel<M>)>,
}

impl<M: Copy + PartialEq, I: Instance<M=M>> Frame<M, I> {
    pub fn new() -> Self {
        Self {
            display_panel: Vec::new(),
        }
    }

    pub fn add_instance(&mut self, instance: I) {
        let layout = instance.layout();
        self.display_panel.push((instance, layout));
    }
}

impl<M: Copy + PartialEq, I: Instance<M=M>> Container<M> for Frame<M, I> {
    fn update(&mut self, el_context: &mut ELContext<'_, M>) -> bool
    {
        let mut is_update = false;
        let mut update_instance = Vec::with_capacity(self.display_panel.len());
        let mut update_index = Vec::with_capacity(self.display_panel.len());
        let mut i = 0;
        for (instance, panel) in self.display_panel.as_mut_slice() {
            is_update = panel.listener(el_context);
            if el_context.message.is_some() {
                instance.update(el_context.message.as_ref().unwrap());
                update_index.push(i);
                // 清除消息，防止重复发送
                el_context.message = None;
            }
            i += 1;
        }
        for index in update_index {
            let (instance, _) = self.display_panel.remove(index);
            let panel = instance.layout();
            update_instance.push((instance, panel));
        }
        self.display_panel.append(&mut update_instance);
        is_update
    }

    fn render(&self, utils: &mut RenderUtil) {
        for (_, panel) in &self.display_panel {
            panel.draw(utils)
        }
    }
}

