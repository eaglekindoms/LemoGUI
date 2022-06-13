use std::sync::Arc;

use glow::HasContext;

use crate::backend::glow_impl::*;
use crate::graphic::base::*;
use crate::graphic::style::Style;

#[derive(Debug)]
pub struct GLPipeline {
    pub context: Arc<glow::Context>,
    pub program: glow::NativeProgram,
    pub vao: glow::VertexArray,
    pub vbo: glow::Buffer,
    pub ebo: glow::Buffer,
    // pub screen_size: glow::NativeUniformLocation,
}

impl GLPipeline {
    pub fn new<V>(context: &Arc<glow::Context>) -> GLPipeline
    where
        V: VertexLayout,
    {
        V::create_render_pipeline(context)
    }

    pub fn draw_indexed(&self, indices_num: i32) {
        unsafe {
            // use shader darw
            self.context.use_program(Some(self.program));
            self.context.bind_vertex_array(Some(self.vao));
            self.context
                .draw_elements(glow::TRIANGLES, indices_num, glow::UNSIGNED_INT, 0);
            self.context.use_program(None);
        }
    }

    pub fn draw_instance(&self) {
        unsafe {
            // use shader darw
            self.context.use_program(Some(self.program));
            self.context.bind_vertex_array(Some(self.vao));
            self.context
                .draw_arrays_instanced(glow::TRIANGLE_STRIP, 0, 4, 1);
            self.context.use_program(None);
        }
    }

    pub fn set_vertex_buffer(&self, vertices: &[u8]) {
        unsafe {
            self.context.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.context
                .buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices, glow::STREAM_DRAW);
            self.context.bind_buffer(glow::ARRAY_BUFFER, None);
        }
    }

    pub fn set_index_buffer(&self, indices: &[u8]) {
        unsafe {
            self.context
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            self.context.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                indices,
                glow::STREAM_DRAW,
            );
            self.context.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }
    }

    pub fn set_screen_size(&self, size: Point<u32>) {
        unsafe {
            self.context.use_program(Some(self.program));
            let screen_size = self
                .context
                .get_uniform_location(self.program, "u_screen_size");
            println!("{:?}", screen_size);
            // See also `uniform_n_i32`, `uniform_n_u32`, `uniform_matrix_4_f32_slice` etc.
            self.context
                .uniform_2_u32(screen_size.as_ref(), size.x / 2, size.y / 2);
            self.context.use_program(None);
        }
    }
}
