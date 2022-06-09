use std::sync::Arc;

use glow::HasContext;
use image::EncodableLayout;

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
            //clear screen
            self.context.clear(glow::COLOR_BUFFER_BIT);
            // use shader darw
            self.context.use_program(Some(self.program));
            self.context.bind_vertex_array(Some(self.vao));
            self.context
                .draw_elements(glow::TRIANGLES, indices_num, glow::UNSIGNED_INT, 0);
            // self.context.draw_arrays(glow::TRIANGLES, 0, 3);
            // self.context.swap_buffers().unwrap();
            // unbind(&self.context, 0);
        }
    }

    pub fn draw(&self) {
        let points = vec![
            Point::new(0.2, -0.6), //0
            Point::new(0.4, -0.6), //1
            Point::new(0.5, -0.4), //2
            Point::new(0.4, -0.2), //3
            Point::new(0.2, -0.2), //4
            Point::new(0.1, -0.4), //5
        ];
        let mut vect = Vec::with_capacity(points.len());
        for i in 0..points.len() {
            vect.push(PointVertex::new(points[i].x, points[i].y, LIGHT_BLUE));
        }
        // when the indices is u16, the cast_slice will make the number two u8.
        // so the type of indices cannot be u16
        let mut indices = PointVertex::get_points_indices(points.len());
        self.set_vertex_buffer(bytemuck::cast_slice(vect.as_slice()));
        self.set_index_buffer(bytemuck::cast_slice(indices.as_slice()));
        self.draw_indexed(indices.len() as i32);
    }
    pub fn set_vertex_buffer(&self, vertices: &[u8]) {
        unsafe {
            self.context.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.context
                .buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices, glow::STREAM_DRAW);
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
        }
    }
}

pub fn unbind(gl_context: &glow::Context, location: u32) {
    unsafe {
        gl_context.bind_buffer(glow::ARRAY_BUFFER, None);
        gl_context.bind_vertex_array(None);
        gl_context.disable_vertex_attrib_array(location);
    }
}
