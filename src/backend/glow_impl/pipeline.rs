use glow::HasContext;
use std::sync::Arc;

use crate::backend::glow_impl::*;

#[derive(Debug)]
pub struct Pipeline {
    pub context: Arc<glow::Context>,
    pub program: glow::NativeProgram,
    pub vao: glow::VertexArray,
    pub vbo: glow::Buffer,
}

impl Pipeline {
    pub fn create_triangle_pipeline(context: &Arc<glow::Context>) -> Pipeline {
        let shader = CShader::new(
            &context,
            include_str!("./shader/triangle.vert"),
            include_str!("./shader/triangle.frag"),
        );
        let program = link_program(&context, &shader);
        let vbo = unsafe { context.create_buffer().unwrap() };
        let vertex_layout = VertexLayoutInfo {
            location: 0,
            vector_size: 3,
            data_type: glow::FLOAT,
            normalized: false,
            stride: 0,
            offset: 0,
        };
        let vao = bind_vertex_attrib(&context, vbo, vertex_layout);
        Pipeline {
            context: Arc::clone(context),
            program,
            vao,
            vbo,
        }
    }

    pub fn draw_arr(&self) {
        unsafe {
            //clear screen
            self.context.clear(glow::COLOR_BUFFER_BIT);
            // use shader darw
            self.context.use_program(Some(self.program));
            self.context.bind_vertex_array(Some(self.vao));
            self.context.draw_arrays(glow::TRIANGLES, 0, 3);
            // self.context.swap_buffers().unwrap();
            // unbind(&self.context, 0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            let firstTriangle: [f32; 9] = [
                -0.9, -0.5, 0.0, // left
                -0.0, -0.5, 0.0, // right
                -0.45, 0.5, 0.0, // top
            ];
            set_buffer_data(
                &self.context,
                self.vbo,
                bytemuck::cast_slice(&firstTriangle),
            );
            self.draw_arr();
        }
    }
}

/// 设置顶点属性，必须在绑定顶点缓冲后执行
pub fn bind_vertex_attrib(
    gl_context: &glow::Context,
    vbo: glow::Buffer,
    vertex_layout: VertexLayoutInfo,
) -> glow::VertexArray {
    let vao = unsafe { gl_context.create_vertex_array().unwrap() };
    unsafe {
        gl_context.bind_vertex_array(Some(vao));
        gl_context.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        //对于数据是紧凑的，stride步长可取0，也可设为(vector_size * std::mem::size_of::<data_type>())
        gl_context.vertex_attrib_pointer_f32(
            vertex_layout.location,
            vertex_layout.vector_size,
            vertex_layout.data_type,
            vertex_layout.normalized,
            vertex_layout.stride,
            vertex_layout.offset,
        );
        gl_context.enable_vertex_attrib_array(vertex_layout.location);
    }
    return vao;
}

pub fn unbind(gl_context: &glow::Context, location: u32) {
    unsafe {
        gl_context.bind_buffer(glow::ARRAY_BUFFER, None);
        gl_context.bind_vertex_array(None);
        gl_context.disable_vertex_attrib_array(location);
    }
}
