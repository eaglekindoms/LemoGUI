use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::Style;
use crate::widget::{Component, ComponentModel};

/// 图形绘制面板控件结构体
pub struct ShapeBoard {
    pub shape_arr: Vec<Box<dyn ShapeGraph>>,
    pub style: Style,
}

impl<M: Clone + PartialEq + 'static> From<ShapeBoard> for Component<M> {
    fn from(shape_board: ShapeBoard) -> Self {
        Component::new(shape_board)
    }
}

impl<M> ComponentModel<M> for ShapeBoard {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        let mut style = self.style;
        for shape in &self.shape_arr {
            paint_brush.draw_shape(shape, style.get_back_color());
            style = Style::default().back_color(LIGHT_BLUE).round();
        }
    }
}
