use crate::graphic::base::*;
use crate::graphic::render_middle::RenderUtil;
use crate::graphic::style::Style;
use crate::widget::{Component, ComponentModel};

/// 图形绘制面板控件结构体
pub struct ShapeBoard {
    pub shape_arr: Vec<Box<dyn ShapeGraph>>,
    pub style: Style,
}

impl<M: Copy + PartialEq + 'static> From<ShapeBoard> for Component<M> {
    fn from(shape_board: ShapeBoard) -> Self {
        Component::new(shape_board)
    }
}

impl<M> ComponentModel<M> for ShapeBoard {
    fn draw(&self, render_utils: &mut RenderUtil) {
        let mut style = self.style;
        for shape in &self.shape_arr {
            shape.to_buffer(render_utils.context, style.get_back_color())
                .render(render_utils, shape.get_type());
            style = Style::default().back_color(LIGHT_BLUE).round();
        }
    }
}
