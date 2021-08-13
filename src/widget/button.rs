use std::fmt::Debug;
use std::option::Option::Some;

use winit::event::*;

use crate::device::event_context::ELContext;
use crate::device::wgpu_context::WGContext;
use crate::graphic::base::image_vertex::TextureVertex;
use crate::graphic::base::shape::{Point, Rectangle, ShapeGraph};
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::RenderUtil;
use crate::graphic::render_middle::texture_buffer::TextureBuffer;
use crate::graphic::style::*;
use crate::widget::component::ComponentModel;
use crate::widget::listener;
use crate::widget::listener::Listener;
use crate::widget::message::{EventType, State};

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
    pub state: Option<State<M>>,
}

impl<'a, M: Copy + PartialEq> Button<M> {
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
            event: EventType::mouse,
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

impl<'a, M: Copy + PartialEq> ComponentModel<M> for Button<M> {
    /// 组件绘制方法实现
    fn draw(&self, wgcontext: &WGContext, render_utils: &mut RenderUtil, glob_pipeline: &PipelineState) {
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
    fn key_listener(&mut self, action_state: ElementState,
                    el_context: &ELContext<'_, M>, virtual_keycode: Option<VirtualKeyCode>) -> bool {
        listener::action_animation::<M>(&mut self.style, action_state,
                                        &el_context.message_channel, &self.state, virtual_keycode)
    }
    fn action_listener(&mut self, action_state: ElementState, el_context: &ELContext<'_, M>) -> bool
    {
        let input = self.size
            .contain_coord(el_context.cursor_pos.unwrap());
        if input {
            listener::action_animation::<M>(&mut self.style, action_state,
                                            &el_context.message_channel, &self.state, None);
        }
        input
    }
}