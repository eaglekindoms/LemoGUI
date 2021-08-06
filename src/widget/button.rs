use std::fmt::Debug;

use winit::event::*;

use crate::device::display_window::WGContext;
use crate::graphic::base::shape::{Point, Rectangle};
use crate::graphic::render_middle::render_function::RenderGraph;
use crate::graphic::style::*;
use crate::widget::component::Component;
use crate::widget::component::ComponentModel;
use crate::widget::listener::Listener;
use crate::widget::message::Message;

/// 按钮控件结构体
#[derive(Debug)]
pub struct Button {
    /// 按钮组件样式
    pub context: Component,
    /// 内容文本
    pub text: String,
    /// 控件状态
    pub message: Option<Message>,
}

impl<'a> Button {
    pub fn new_with_style<S: Into<String>>(rect: Rectangle, style: Style, text: S) -> Self {
        log::info!("create the Button obj use new");
        Self {
            context: Component::new(rect, style),
            text: text.into(),
            message: None,
        }
    }

    pub fn new<S: Into<String>>(pos: Point<f32>, text: S) -> Self {
        let text = text.into();
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32, 40);
        log::info!("create the Button obj use default");
        Self {
            context: Component::new(rect, Style::default()),
            text,
            message: None,
        }
    }

    /// 更新状态
    pub fn message(mut self, state: Option<Message>) -> Self {
        self.message = state;
        self
    }

    /// 更新内容
    pub fn update_content<S: Into<String>>(&mut self, text: S) {
        self.text = text.into();
        self.context.set_is_redraw(true);
    }

    // pub fn set_style(&mut self, style: Style) {
    //     self.style = style;
    // }
}

impl<'a> ComponentModel for Button {
    fn to_graph(&mut self, wgcontext: &WGContext) -> Option<&RenderGraph> {
        let text = &self.text;
        Some(self.context.to_graph(text, wgcontext))
    }
}

impl<'a> Listener for Button {
    fn key_listener(&mut self, virtual_keycode: Option<VirtualKeyCode>) -> bool {
        let mut input = false;
        if let Some(state) = self.message.as_ref() {
            if virtual_keycode == state.get_key() {
                if let Some(callback) = state.get_key_callback() {
                    callback();
                }
                input = true;
            }
        }
        input
    }
    fn action_listener(&mut self, position: Point<f32>) -> bool {
        let input = self.context
            .get_size()
            .contain_coord(position);
        if input {
            if let Some(state) = self.message.as_ref() {
                if let Some(callback) = state.get_action_callback() {
                    callback();
                }
            }
        }
        input
    }
}