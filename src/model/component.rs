use wgpu::Device;

use crate::device::display_window::DisplayWindow;
use crate::device::listener::Listener;
use crate::graphic::render_type::buffer_state::VertexBuffer;
use crate::graphic::render_type::render_function::RenderGraph;
use crate::graphic::render_type::texture_state::TextureState;
use crate::graphic::shape::point2d::RGBA;
use crate::graphic::shape::rectangle::Rectangle;
use crate::graphic::shape::round_rectangle::RectState;
use crate::graphic::shape::texture_point::TextState;

/// 组件属性：矩形，背景颜色，聚焦颜色，文字颜色，文本内容
#[derive(Debug)]
pub struct Component<'a, L: Listener + ?Sized> {
    size: Rectangle,
    font_color: RGBA,
    background_color: RGBA,
    border_color: RGBA,
    hover_color: RGBA,
    text: &'a str,
    listener: Option<Box<L>>,
}

impl<'a> Component<'a, dyn Listener> {
    pub fn new(rect: Rectangle, font_color: RGBA, background_color: RGBA,
               border_color: RGBA, hover_color: RGBA,
               text: &'a str, listener: Box<Listener>) -> Self {
        Self {
            size: rect,
            font_color,
            background_color,
            border_color,
            hover_color,
            text,
            listener: Option::from(listener),
        }
    }

    pub fn default(rect: Rectangle, font_color: RGBA, background_color: RGBA, border_color: RGBA, hover_color: RGBA, text: &'a str) -> Self {
        Self {
            size: rect,
            font_color,
            background_color,
            border_color,
            hover_color,
            text,
            listener: None,
        }
    }

    pub fn to_graph(&self, display_window: &DisplayWindow) -> RenderGraph {
        let vertex_buffer = VertexBuffer::create_tex_vertex_buf(&display_window.device, &display_window.sc_desc, &self.size);
        let shape_vertex_buffer = VertexBuffer::create_background_buf(&display_window.device, &display_window.sc_desc, &self.size, self.background_color);
        let hover_vertex_buffer = VertexBuffer::create_background_buf(&display_window.device, &display_window.sc_desc, &self.size, self.hover_color);
        let boder_vertex_buffer = VertexBuffer::create_border_buf(&display_window.device, &display_window.sc_desc, &self.size, self.border_color);
        let texture_state = TextureState::create_text_texture(&display_window.device, &display_window.queue, self.text);

        RenderGraph {
            vertex_buffer,
            back_buffer: shape_vertex_buffer,
            hover_buffer: Some(hover_vertex_buffer),
            border_buffer: boder_vertex_buffer,
            context_buffer: texture_state,
        }
    }
}

pub trait ComponentModel {
    fn set_index(&mut self, index: usize);
    fn get_index(&self) -> Option<usize>;
    fn to_graph(&self, display_window: &DisplayWindow) -> RenderGraph;
}