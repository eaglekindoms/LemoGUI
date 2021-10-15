use std::fmt::Debug;
use std::option::Option::Some;

use winit::event::*;
use winit::window::CursorIcon;

use crate::device::ELContext;
use crate::graphic::base::*;
use crate::graphic::render_middle::RenderUtil;
use crate::graphic::render_middle::TextureVertex;
use crate::graphic::style::*;
use crate::widget::component;
use crate::widget::component::ComponentModel;
use crate::widget::message::{EventType, State};

/// 按钮控件结构体
#[derive(Debug)]
pub struct TextInput<M: Copy> {
    /// 组件尺寸
    pub size: Rectangle,
    /// 组件样式
    pub style: Style,
    /// 内容文本
    pub text: String,
    /// 控件状态
    pub state: Option<State<M>>,
}

impl<'a, M: Copy + PartialEq> TextInput<M> {
    pub fn new_with_style<S: Into<String>>(mut rect: Rectangle, style: Style, text: S) -> Self {
        log::info!("create the Button obj use new");
        Self {
            size: rect.set_style(style),
            text: text.into(),
            state: None,
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
            state: None,
        }
    }

    /// 更新状态
    pub fn action(mut self, message: M) -> Self {
        self.state = Some(State {
            event: EventType::Mouse,
            message: Some(message),
        });
        self
    }

    pub fn match_message(&self, des_m: &M) -> bool {
        if self.state.is_some() {
            self.state.as_ref().unwrap().match_message(des_m)
        } else {
            false
        }
    }
}

impl<'a, M: Copy + PartialEq> ComponentModel<M> for TextInput<M> {
    /// 组件绘制方法实现
    fn draw(&self, render_utils: &mut RenderUtil) {
        let image_vertex_buffer =
            TextureVertex::new
                (&render_utils.context.device, render_utils.context.get_surface_size(), &self.size);
        let back_buffer = self.size.to_buffer(render_utils.context, self.style.get_display_color());
        let font_buffer = render_utils.context.get_text_buffer(self.text.as_str());
        back_buffer.render(render_utils, self.size.get_type());
        image_vertex_buffer.render_t(render_utils, &font_buffer);
    }
    fn key_listener(&mut self, action_state: ElementState,
                    el_context: &ELContext<'_, M>, virtual_keycode: Option<VirtualKeyCode>) -> bool {
        component::action_animation::<M>(&mut self.style, action_state,
                                         &el_context.message_channel, &self.state, virtual_keycode)
    }
    fn hover_listener(&mut self, el_context: &ELContext<'_, M>) -> bool
    {
        if el_context.cursor_pos.is_none() { return false; }
        let input = self.size
            .contain_coord(el_context.cursor_pos.unwrap());
        if input {
            el_context.window.set_cursor_icon(CursorIcon::Text);
            // listener::action_animation::<M>(&mut self.style, action_state,
            //                                 &el_context.message_channel, &self.state, None);
        } else {
            el_context.window.set_cursor_icon(CursorIcon::Default);
        }
        input
    }
}