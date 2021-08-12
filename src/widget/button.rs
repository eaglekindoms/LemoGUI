use std::fmt::Debug;
use std::option::Option::Some;

use winit::event::*;
use winit::event::VirtualKeyCode::Key1;
use winit::event_loop::EventLoopProxy;

use crate::device::event_context::ELContext;
use crate::device::wgpu_context::WGContext;
use crate::graphic::base::image_vertex::TextureVertex;
use crate::graphic::base::shape::{Point, Rectangle, ShapeGraph};
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::RenderUtil;
use crate::graphic::render_middle::texture_buffer::TextureBuffer;
use crate::graphic::style::*;
use crate::widget::component::ComponentModel;
use crate::widget::listener::Listener;
use crate::widget::message::Message;

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
    pub message: Option<Message>,
    pub custom: Option<M>,
    pub list: Option<M>,
}

impl<'a, M: Copy + PartialEq> Button<M> {
    pub fn new_with_style<S: Into<String>>(mut rect: Rectangle, style: Style, text: S) -> Self {
        log::info!("create the Button obj use new");
        Self {
            size: rect.set_style(style),
            text: text.into(),
            message: None,
            style,
            custom: None,
            list: None,
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
            custom: None,
            list: None,
        }
    }

    /// 更新状态
    pub fn message(mut self, message: Message) -> Self {
        self.message = Some(message);
        self
    }

    /// 更新内容
    pub fn update_content<S: Into<String>>(&mut self, text: S) {
        self.text = text.into();
    }

    pub fn set_custom(mut self, custom: M) -> Self {
        self.custom = Some(custom);
        self
    }
    pub fn set_list(mut self, list: M) -> Self {
        self.list = Some(list);
        self
    }

    pub fn get_customer(&self) -> M {
        self.custom.unwrap()
    }
}

impl<'a, M: Copy + PartialEq> ComponentModel<M> for Button<M> {
    /// 组件绘制方法实现
    fn draw(&mut self, wgcontext: &WGContext, render_utils: &mut RenderUtil, glob_pipeline: &PipelineState) {
        let image_vertex_buffer =
            TextureVertex::new
                (&wgcontext.device, &wgcontext.sc_desc, &self.size);
        let back_buffer = self.size.to_buffer(wgcontext, self.style.get_display_color());
        let font_buffer =
            TextureBuffer::create_font_image
                (&wgcontext.device,
                 &wgcontext.queue, self.style.get_font_color(), self.text.as_str());
        back_buffer.render(render_utils, glob_pipeline, self.size.get_type());
        image_vertex_buffer.render_t(render_utils, &font_buffer, &glob_pipeline);
    }
}

impl<'a, M: Copy + PartialEq> Listener<M> for Button<M> {
    fn key_listener(&mut self, virtual_keycode: Option<VirtualKeyCode>) -> bool {
        let mut input = false;
        if let Some(state) = self.message.as_ref() {
            if virtual_keycode == state.get_key() {
                // if let Some(callback) = state.get_key_callback() {
                //     callback();
                // }
                input = true;
            }
        }
        input
    }
    fn action_listener(&mut self, state: ElementState, el_context: &ELContext<'_, M>) -> bool
    {
        let input = self.size
            .contain_coord(el_context.cursor_pos.unwrap());
        if input {
            let hover_color = self.style.get_hover_color();
            let back_color = self.style.get_back_color();
            if state == ElementState::Pressed {
                self.style.display_color(hover_color);
                if let Some(state) = self.message.as_ref() {
                    if self.custom.is_some() {
                        let custom = self.get_customer();
                        el_context.event_loop_proxy.send_event(custom).ok();
                    }
                }
            } else if state == ElementState::Released {
                self.style.display_color(back_color);
            }
        }
        input
    }

    fn sub_listener(&mut self, broadcast: &M) -> bool {
        if let Some(list) = self.list.as_ref() {
            if *list == *broadcast {
                self.text = "nnnn".to_string();
                return true;
            }
        }
        return false;
    }
}