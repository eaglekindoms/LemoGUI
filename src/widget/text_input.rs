use winit::window::CursorIcon;

use crate::device::ELContext;
use crate::graphic::base::*;
use crate::graphic::render_middle::RenderUtil;
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
    pub fn new_with_style<S: Into<String>, MT>(mut rect: Rectangle,
                                               style: Style, text: S, rec: MT) -> Self
        where MT: 'static + Fn(String) -> M {
        log::info!("create the Button obj use new");
        Self {
            size: rect.set_style(style),
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
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32, 40);
        log::info!("create the Button obj use default");
        Self {
            size: rect,
            style: Style::default(),
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
    /// 组件绘制方法实现
    fn draw(&self, render_utils: &mut RenderUtil, font_map: &mut GCharMap) {
        render_utils.draw_rect(&self.size, WHITE);
        render_utils.draw_text(font_map, &self.size, self.text.as_str(), self.style.get_font_color());
    }

    fn hover_listener(&mut self, el_context: &ELContext<'_, M>) -> bool
    {
        let input = self.size
            .contain_coord(el_context.cursor_pos);
        if input {
            el_context.window.set_cursor_icon(CursorIcon::Text);
        } else {
            el_context.window.set_cursor_icon(CursorIcon::Default);
        }
        input
    }
    fn received_character(&mut self, _el_context: &ELContext<'_, M>, c: char) -> bool {
        if self.hover_listener(_el_context) {
            println!("ime: {:?}", c);
            if c == '\u{8}' {
                self.text.pop();
            } else {
                self.text.push(c);
            }
            _el_context.send_message((self.text_receive)(self.text.clone()));
        }
        true
    }
}
