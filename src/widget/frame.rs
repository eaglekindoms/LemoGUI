use crate::event::EventContext;
use crate::graphic::base::GCharMap;
use crate::graphic::render_api::PaintBrush;
use crate::instance::*;
use crate::widget::*;

/// 窗口帧结构体
/// 作用：用作gui控件的容器
pub struct Frame<M: PartialEq + Clone, I: Instance<M = M>> {
    pub display_panel: Vec<(I, Panel<M>)>,
}

impl<M: Clone + PartialEq, I: Instance<M = M>> Frame<M, I> {
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

impl<M: Clone + PartialEq, I: Instance<M = M>> ComponentModel<M> for Frame<M, I> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        for (_, panel) in &self.display_panel {
            panel.draw(paint_brush, font_map)
        }
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let mut is_update = false;
        let mut updated_instance: Vec<(I, Panel<M>)> = Vec::with_capacity(self.display_panel.len());
        let mut updated_index = Vec::with_capacity(self.display_panel.len());
        let mut i = 0;
        for (instance, panel) in self.display_panel.as_mut_slice() {
            if panel.listener(event_context) {
                is_update = true;
            }
            if event_context.get_message().is_some() {
                instance.update(event_context.get_message().unwrap());
                updated_index.push(i);
                // 清除消息，防止重复发送
                event_context.set_message(None);
            }
            i += 1;
        }
        for index in updated_index {
            let (instance, _) = self.display_panel.remove(index);
            let panel = instance.layout();
            updated_instance.push((instance, panel));
        }
        self.display_panel.append(&mut updated_instance);
        is_update
    }
}
