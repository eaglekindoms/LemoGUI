use std::fmt::Debug;

use winit::event::*;

use crate::device::display_window::WGContext;
use crate::graphic::base::shape::{Point, Rectangle};
use crate::graphic::render_middle::render_function::RenderGraph;
use crate::graphic::style::*;
use crate::widget::component::Component;
use crate::widget::component::ComponentModel;
use crate::widget::listener::{Listener, Message};

/// 按钮控件结构体
#[derive(Debug)]
pub struct Button {
    /// 按钮组件样式
    pub context: Component,
    /// 内容文本
    pub text: String,
    /// 控件状态
    pub state: Option<Message>,
}

impl<'a> Button {
    pub fn new_with_style<S: Into<String>>(rect: Rectangle, style: Style, text: S) -> Self {
        log::info!("create the Button obj use new");
        Self {
            context: Component::new(rect, style),
            text: text.into(),
            state: None,
        }
    }

    pub fn new<S: Into<String>>(pos: Point<f32>, text: S) -> Self {
        let text = text.into();
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32, 40);
        log::info!("create the Button obj use default");
        Self {
            context: Component::new(rect, Style::default()),
            text,
            state: None,
        }
    }

    /// 更新状态
    pub fn update_state(mut self, state: Option<Message>) -> Self {
        self.state = state;
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
    fn key_listener(&mut self, event: &WindowEvent) -> bool {
        // log::info!("---button--- {:?}", event);
        let mut input = false;
        match self.state.as_ref() {
            Some(state) => {
                let key = state.get_key();
                match event {
                    WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state,
                            virtual_keycode,
                            ..
                        },
                        ..
                    }if virtual_keycode.as_ref() == key => {
                        if *state == ElementState::Pressed {
                            let text = self.text.as_str().to_owned() + "2";
                            self.update_content(text);
                            input = true;
                        } else if *state == ElementState::Released {}
                    }

                    _ => {}
                }
            }
            None => {}
        }
        input
    }
}