use winit::window::CursorIcon;

use crate::device::EventContext;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::{Component, ComponentModel};

/// 按钮控件结构体
#[allow(missing_debug_implementations)]
pub struct TextInput<M: Clone> {
    /// 组件尺寸
    pub size: Rectangle,
    /// 组件样式
    pub style: Style,
    /// 内容文本
    pub text: String,
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
            size: rect,
            text: text.into(),
            state: None,
            style,
            is_focus: false,
            text_receive: Box::new(rec),
        }
    }

    pub fn new<S: Into<String>, MT>(pos: Point<f32>, text: S, rec: MT) -> Self
        where MT: 'static + Fn(String) -> M {
        let text = text.into();
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32 + 10, 40);
        Self {
            size: rect,
            style: Style::default().back_color(WHITE),
            text,
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
        let shape: Box<dyn ShapeGraph> = Box::new(self.size);
        paint_brush.draw_shape(&shape, self.style);
        paint_brush.draw_text(font_map, &self.size, self.text.as_str(), self.style.get_font_color());
    }

    fn hover_listener(&mut self, event_context: &EventContext<'_, M>) -> bool
    {
        let input = self.size
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
            println!("ime: {:?}", c);
            if c == '\u{8}' {
                self.text.pop();
            } else {
                self.text.push(c);
            }
            event_context.send_message((self.text_receive)(self.text.clone()));
        }
        true
    }
}
