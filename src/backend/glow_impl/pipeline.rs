use glow::HasContext;

use crate::backend::glow_impl::*;

#[derive(Debug)]
pub struct Pipeline {
    pub program: glow::NativeProgram,
}

impl Pipeline {
    pub fn create_triangle_pipeline(gl: &glow::Context) -> Pipeline {

        let shader = CShader::new(
            gl,
            include_str!("./shader/triangle.vert"),
            include_str!("./shader/triangle.frag"),
        );
        let program=link_program(gl,&shader);
        Pipeline {
            program
        }
    }

    pub fn draw(&self, context: &GLGPUContext) {
        unsafe {
            context.gl_context.use_program(Some(self.program));
            context.gl_context.clear(glow::COLOR_BUFFER_BIT);
            context.gl_context.draw_arrays(glow::TRIANGLES, 0, 3);
            context.gl_window.swap_buffers().unwrap();
        }
    }
}