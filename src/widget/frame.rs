use crate::device::Container;
use crate::device::EventContext;
use crate::graphic::base::{DEFAULT_FONT_SIZE, GCharMap};
use crate::graphic::render_middle::RenderUtil;
use crate::widget::{Instance, Panel};
use crate::widget::component::ComponentModel;

/// 窗口帧结构体
/// 作用：用作gui控件的容器
pub struct Frame<M: PartialEq + Clone, I: Instance<M=M>> {
    pub display_panel: Vec<(I, Panel<M>)>,
    /// 字体缓冲
    pub font_map: GCharMap,

}

impl<M: Clone + PartialEq, I: Instance<M=M>> Frame<M, I> {
    pub fn new(font_path: String) -> Self {
        let font_map = GCharMap::new(font_path, DEFAULT_FONT_SIZE);
        Self {
            display_panel: Vec::new(),
            font_map,
        }
    }

    pub fn add_instance(&mut self, instance: I) {
        let layout = instance.layout();
        self.display_panel.push((instance, layout));
    }
}

impl<M: Clone + PartialEq, I: Instance<M=M>> Container<M> for Frame<M, I> {
    fn update(&mut self, event_context: &mut EventContext<'_, M>) -> bool
    {
        let mut is_update = false;
        let mut updated_instance: Vec<(I, Panel<M>)> = Vec::with_capacity(self.display_panel.len());
        let mut updated_index = Vec::with_capacity(self.display_panel.len());
        let mut i = 0;
        for (instance, panel) in self.display_panel.as_mut_slice() {
            if panel.listener(event_context) {
                is_update = true;
            }
            if event_context.message.is_some() {
                instance.update(event_context.message.as_ref().unwrap());
                updated_index.push(i);
                // 清除消息，防止重复发送
                event_context.message = None;
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

    fn render(&mut self, utils: &mut RenderUtil) {
        for (_, panel) in &self.display_panel {
            panel.draw(utils, &mut self.font_map)
        }
    }
}

