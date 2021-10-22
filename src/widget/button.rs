use std::fmt::Debug;
use std::option::Option::Some;

use crate::device::ELContext;
use crate::graphic::base::*;
use crate::graphic::render_middle::RenderUtil;
use crate::graphic::style::*;
use crate::widget::{Component, ComponentModel};

/// 按钮控件结构体
#[derive(Debug)]
pub struct Button<M: Copy> {
    /// 组件尺寸
    pub size: Rectangle,
    /// 组件样式
    pub style: Style,
    /// 内容文本
    pub text: String,
    /// 控件状态
    pub message: Option<M>,
}

impl<'a, M: Copy + PartialEq> Button<M> {
    pub fn new_with_style<S: Into<String>>(mut rect: Rectangle, style: Style, text: S) -> Self {
        log::info!("create the Button obj use new");
        Self {
            size: rect.set_style(style),
            text: text.into(),
            message: None,
            style,
        }
    }

    pub fn new<S: Into<String>>(pos: Point<f32>, text: S) -> Self {
        let text = text.into();
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32, 40);
        log::info!("create the Button obj use default");
        Self {
            size: rect,
            style: Style::default(),
            text,
            message: None,
        }
    }

    /// 更新状态
    pub fn action(mut self, message: M) -> Self {
        self.message = Some(message);
        self
    }
}

impl<M: Copy + PartialEq + 'static> From<Button<M>> for Component<M> {
    fn from(button: Button<M>) -> Self {
        Component::new(button)
    }
}

impl<'a, M: Copy + PartialEq> ComponentModel<M> for Button<M> {
    /// 组件绘制方法实现
    fn draw(&self, render_utils: &mut RenderUtil) {
        render_utils.draw_rect(&self.size, self.style.get_display_color());
        render_utils.draw_text(&self.size, self.text.as_str(), self.style.get_font_color());
    }
    fn key_listener(&mut self, el_context: &ELContext<'_, M>) -> bool {
        false
    }
    fn action_listener(&mut self, el_context: &ELContext<'_, M>) -> bool
    {
        el_context.action_animation(&mut self.style, &self.size, self.message)
    }
}