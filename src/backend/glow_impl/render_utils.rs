use glow::HasContext;

use crate::graphic::base::{GCharMap, ImageRaw, Rectangle, ShapeGraph, RGBA};
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::Style;

/// gl render tool
#[derive(Debug)]
pub struct GRenderUtil {
    gl: glow::Context,
}

impl GRenderUtil {
    pub fn new() {}
}

impl PaintBrush for GRenderUtil {
    fn clear_frame(&mut self, color: RGBA) {
        unsafe {
            self.gl.clear_color(color.0, color.1, color.2, color.3);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    fn draw_shape(&mut self, shape: &Box<dyn ShapeGraph>, shape_style: Style) {
        todo!()
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
