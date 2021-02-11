use crate::backend::shape::{Rectangle, RGBA};
use crate::backend::buffer_state::VertexBuffer;
use crate::backend::texture_state::TextureState;
use crate::backend::global_setting::GlobalState;
use crate::backend::render::*;
use crate::backend::pipeline_state::PipelineState;

/// 按钮属性：矩形，背景颜色，聚焦颜色，文字颜色，文本内容
pub struct Button<'a> {
    size: &'a Rectangle,
    font_color: RGBA,
    background_color: RGBA,
    border_color: RGBA,
    hover_color: RGBA,
    text: &'a str,
}

impl<'a> Button<'a> {
    pub fn new(rect: &'a Rectangle, font_color: RGBA, background_color: RGBA, border_color: RGBA, hover_color: RGBA, text: &'a str) -> Self {
        Self {
            size: rect,
            font_color,
            background_color,
            border_color,
            hover_color,
            text,
        }
    }
    pub fn default(rect: &'a Rectangle, text: &'a str) -> Self {
        let font_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let background_color = RGBA([0.8, 0.8, 0.8, 1.0]);
        let border_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let hover_color = RGBA([0.5, 0.0, 0.5, 0.5]);
        Self::new(rect, font_color, background_color, border_color, hover_color, text)
    }
    pub fn to_graph(&self, global_state: &'a GlobalState) -> RenderGraph {
        let vertex_buffer = VertexBuffer::create_tex_vertex_buf(global_state, self.size);
        let shape_vertex_buffer = VertexBuffer::create_background_buf(global_state, self.size, self.background_color);
        let hover_vertex_buffer = VertexBuffer::create_background_buf(global_state, self.size, self.hover_color);
        let boder_vertex_buffer = VertexBuffer::create_border_buf(global_state, self.size, self.border_color);
        let texture_state = TextureState::create_text_texture(global_state, self.text);

        RenderGraph {
            vertex_buffer,
            back_buffer: shape_vertex_buffer,
            hover_buffer: Some(hover_vertex_buffer),
            border_buffer: boder_vertex_buffer,
            context_buffer: texture_state,
        }
    }
}
