use crate::event::{Cursor, EventContext};
use crate::graphic::base::{GCharMap, Point};
use crate::graphic::render_api::PaintBrush;
use crate::instance::*;
use crate::widget::*;

/// 窗口帧结构体
/// 作用：用作gui控件的容器
pub struct Frame<M: PartialEq + Clone, I: Instance<M = M>> {
    pub display_panel: Vec<(I, Panel<M>)>,
    needs_layout: bool,
}

impl<M: Clone + PartialEq, I: Instance<M = M>> Frame<M, I> {
    pub fn new() -> Self {
        Self {
            display_panel: Vec::new(),
            needs_layout: false,
        }
    }

    pub fn add_instance(&mut self, instance: I) {
        let layout = instance.layout();
        self.display_panel.push((instance, layout));
    }

    fn relayout_dirty(&mut self) {
        if !self.needs_layout {
            return;
        }
        self.needs_layout = false;
        let mut rebuilt: Vec<(I, Panel<M>)> = Vec::with_capacity(self.display_panel.len());
        while let Some((instance, _)) = self.display_panel.pop() {
            let panel = instance.layout();
            rebuilt.push((instance, panel));
        }
        rebuilt.reverse();
        self.display_panel = rebuilt;
    }
}

impl<M: Clone + PartialEq, I: Instance<M = M>> ComponentModel<M> for Frame<M, I> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        for (_, panel) in &self.display_panel {
            panel.draw(paint_brush, font_map)
        }
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        event_context.set_cursor_icon(Cursor::Default);
        let mut is_update = false;
        for (instance, panel) in self.display_panel.as_mut_slice() {
            if panel.listener(event_context) {
                is_update = true;
            }
            if event_context.get_message().is_some() {
                instance.update(event_context.get_message().unwrap());
                event_context.set_message(None);
                self.needs_layout = true;
                is_update = true;
            }
        }
        is_update
    }

    fn commit(&mut self) {
        self.relayout_dirty();
    }

    fn ime_caret(&self) -> Option<(Point<f32>, f32)> {
        for (_, panel) in &self.display_panel {
            if let Some(p) = panel.ime_caret() {
                return Some(p);
            }
        }
        None
    }
}
