use crate::device::display_window::WGContext;
use crate::device::listener::Listener;
use crate::graphic::base::color::RGBA;
use crate::graphic::base::image2d::{TextureBuffer, TextureVertex};
use crate::graphic::base::point2d::PointVertex;
use crate::graphic::base::rectangle::Rectangle;
use crate::graphic::render_middle::render_function::RenderGraph;
use crate::graphic::render_middle::vertex_buffer::VertexBuffer;

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
               text: &'a str, listener: Box<dyn Listener>) -> Self {
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

    pub fn to_graph(&self, display_window: &WGContext) -> RenderGraph {
        let vertex_buffer = VertexBuffer::create_vertex_buf::<TextureVertex>(&display_window.device, &display_window.sc_desc, &self.size, &[0, 2, 1, 3], self.border_color);
        let shape_vertex_buffer = VertexBuffer::create_vertex_buf::<PointVertex>(&display_window.device, &display_window.sc_desc, &self.size, &[0, 2, 1, 3], self.background_color);
        let hover_vertex_buffer = VertexBuffer::create_vertex_buf::<PointVertex>(&display_window.device, &display_window.sc_desc, &self.size, &[0, 2, 1, 3], self.hover_color);
        let boder_vertex_buffer = VertexBuffer::create_vertex_buf::<PointVertex>(&display_window.device, &display_window.sc_desc, &self.size, &[0, 1, 3, 2, 0], self.border_color);
        let texture_state = TextureBuffer::create_text_texture(&display_window.device, &display_window.queue, self.text);

        // let round_vertex_buffer = RectVertex
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
    fn to_graph(&self, wgcontext: &WGContext) -> RenderGraph;
}