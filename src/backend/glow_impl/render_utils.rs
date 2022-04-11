use glow::HasContext;

use crate::backend::glow_impl::{create_program, create_shader, GLGPUContext, ShaderType};
use crate::graphic::base::{GCharMap, ImageRaw, Rectangle, ShapeGraph, RGBA};
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::Style;

/// gl render tool
#[derive(Debug)]
pub struct GRenderUtil<'a> {
    /// 图形渲染上下文
    pub context: &'a mut GLGPUContext,
}

impl<'a> GRenderUtil<'a> {
    pub fn new(gpu_context: &'a mut GLGPUContext) -> Self {
        Self {
            context: gpu_context,
        }
    }
}

impl<'a> PaintBrush for GRenderUtil<'a> {
    fn clear_frame(&mut self, color: RGBA) {
        self.context.clear_frame(color)
    }

    fn draw_shape(&mut self, shape: &Box<dyn ShapeGraph>, shape_style: Style) {
        let program = create_program(&self.context.gl_context);
        let s_vert = create_shader(
            &self.context.gl_context,
            &program,
            ShaderType::Vertex,
            include_str!("./shader/triangle.vert"),
        );
        let s_frag = create_shader(
            &self.context.gl_context,
            &program,
            ShaderType::Fragment,
            include_str!("./shader/triangle.frag"),
        );
        unsafe {
            self.context.gl_context.use_program(Some(program));
            self.context.gl_context.clear(glow::COLOR_BUFFER_BIT);
            self.context.gl_context.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }

    fn draw_text(
        &mut self,
        font_map: &mut GCharMap,
        text_rect: &Rectangle,
        text: &str,
        text_color: RGBA,
    ) {
        todo!()
    }

    fn draw_image(&mut self, image_rect: &Rectangle, image: ImageRaw) {
        todo!()
    }
}
