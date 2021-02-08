use crate::backend::shape::{Rectangle, RGBA};
use crate::backend::bufferState::VertexBuffer;
use crate::backend::globeSetting::GlobeState;

/// 按钮属性：矩形，背景颜色，聚焦颜色，文字颜色，文本内容
struct Button<'a> {
    size: Rectangle,
    font_color: RGBA,
    background_color: RGBA,
    border_color: RGBA,
    hover_color: RGBA,
    text: &'a str,
}

struct ButtonGraph {
    vertex_buffer: VertexBuffer,
    shape_vertex_buffer: VertexBuffer,
    boder_vertex_buffer: VertexBuffer,
}

impl<'a> Button {
    pub fn generate_graph_setting(&self, globe_state: &'a GlobeState) -> ButtonGraph {
        let vertex_buffer = VertexBuffer::create_tex_vertex_buf(globe_state, &rect);
        let shape_vertex_buffer = VertexBuffer::create_shape_vertex_buf(globe_state, &rect);
        let boder_vertex_buffer = VertexBuffer::create_border_vertex_buf(globe_state, &rect);
        ButtonGraph{
            vertex_buffer,
            shape_vertex_buffer,
            boder_vertex_buffer
        }
    }
}