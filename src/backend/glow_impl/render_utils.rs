use crate::adapter::TextureBuffer;

use crate::backend::glow_impl::*;
use crate::graphic::base::*;
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
        // let pipeline = GLPipeline::new::<PointVertex>(&self.context.gl_context);
        // pipeline.draw();
        // let rect_pipeline = GLPipeline::new::<RectVertex>(&self.context.gl_context);
        // rect_pipeline.draw_instance(self.context.window.inner_size().into());
        // rect_pipeline.draw_instance(Point::new(800, 600));
        let shape_buffer = shape.to_buffer(self.context, shape_style);
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

    fn set_texture(&mut self, image: ImageRaw) -> TextureBuffer {
        todo!()
    }

    fn draw_image(&mut self, image_rect: &Rectangle, image: ImageRaw) {
        todo!()
    }
}
