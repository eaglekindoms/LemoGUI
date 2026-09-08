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

    /// 按样式绘制一段文本，返回推进宽度
    fn draw_styled_text(
        &mut self,
        font_map: &mut GCharMap,
        origin: Point<f32>,
        text: &str,
        style: TextStyle,
    ) -> f32;

    /// 绘制图像
    fn draw_image(&mut self, image_rect: &Rectangle, image: ImageRaw);

    /// 压入裁剪矩形，与当前栈求交
    fn push_clip(&mut self, rect: Rectangle);

    /// 弹出最近一次裁剪
    fn pop_clip(&mut self);
}
