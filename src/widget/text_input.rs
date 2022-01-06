use winit::window::CursorIcon;

use crate::device::EventContext;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::{Component, ComponentModel, Label};

/// 按钮控件结构体
#[allow(missing_debug_implementations)]
pub struct TextInput<M: Clone> {
    /// 组件面板
    pub text_label: Label,
    /// 控件状态
    pub state: Option<M>,
    pub text_receive: Box<dyn Fn(String) -> M>,
    ///是否聚焦
    pub is_focus: bool,
}

impl<'a, M: Clone + PartialEq> TextInput<M> {
    pub fn new_with_style<S: Into<String>, MT>(rect: Rectangle,
                                               style: Style, text: S, rec: MT) -> Self
        where MT: 'static + Fn(String) -> M {
        Self {
            text_label: Label::new_text_label(rect, style, text.into()),
            state: None,
            is_focus: false,
            text_receive: Box::new(rec),
        }
    }

    pub fn new<S: Into<String>, MT>(pos: Point<f32>, text: S, rec: MT) -> Self
        where MT: 'static + Fn(String) -> M {
        let text = text.into();
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32 + 10, 40);
        let style = Style::default().back_color(WHITE);
        Self {
            text_label: Label::new_text_label(rect, style, text),
            state: None,
            text_receive: Box::new(rec),
            is_focus: false,
        }
    }
}

impl<M: Clone + PartialEq + 'static> From<TextInput<M>> for Component<M> {
    fn from(text_input: TextInput<M>) -> Self {
        Component::new(text_input)
    }
}

impl<'a, M: Clone + PartialEq> ComponentModel<M> for TextInput<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        self.text_label.draw(paint_brush, font_map)
    }

    fn hover_listener(&mut self, event_context: &EventContext<'_, M>) -> bool
    {
        let input = self.text_label.size
            .contain_coord(event_context.cursor_pos);
        if input {
            event_context.window.set_cursor_icon(CursorIcon::Text);
        } else {
            event_context.window.set_cursor_icon(CursorIcon::Default);
        }
        input
    }
    fn received_character(&mut self, event_context: &EventContext<'_, M>, c: char) -> bool {
        if self.hover_listener(event_context) {
            log::debug!("ime: {:?}", c);
            if let Some(text) = &mut self.text_label.text {
                if c == '\u{8}' {
                    text.pop();
                } else {
                    text.push(c);
                }
                event_context.send_message((self.text_receive)(text.clone()));
            }
        }
        true
    }
}
