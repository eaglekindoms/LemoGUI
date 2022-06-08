use crate::adapter::TextureBuffer;
use crate::graphic::base::*;
use crate::graphic::style::Style;

/// 定义绘图接口，描述基本绘图方法
pub trait PaintBrush {
    /// 由指定颜色清空屏幕
    fn clear_frame(&mut self, color: RGBA);

    /// 绘制图形
    fn draw_shape(&mut self, shape: &Box<dyn ShapeGraph>, shape_style: Style);

    /// 绘制文本
    fn draw_text(
        &mut self,
        font_map: &mut GCharMap,
        text_rect: &Rectangle,
        text: &str,
        text_color: RGBA,
    );

    /// 生成纹理缓冲数据
    fn set_texture(&mut self, image: ImageRaw) -> TextureBuffer;
    /// 绘制图像
    fn draw_image(&mut self, image_rect: &Rectangle, image: ImageRaw);
}
