use glow::HasContext;

#[derive(Debug)]
pub struct VertexLayoutInfo {
    pub location: u32,
    pub vector_size: i32,
    //GL_FLOAT,GL_UNSIGNED_BYTE
    pub data_type: u32,
    pub normalized: bool,
    pub stride: i32,
    pub offset: i32,
}

pub fn set_buffer_data(gl_context: &glow::Context, vbo: glow::Buffer, data: &[u8]) {
    unsafe {
        gl_context.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl_context.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STREAM_DRAW);
    }
}
