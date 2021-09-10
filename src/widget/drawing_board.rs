use crate::device::WGContext;
use crate::graphic::base::*;
use crate::graphic::render_middle::PipelineState;
use crate::graphic::render_middle::RenderUtil;
use crate::graphic::style::Style;
use crate::widget::component::ComponentModel;

/// 图形绘制面板控件结构体
pub struct ShapeBoard {
    pub shape_arr: Vec<Box<dyn ShapeGraph>>,
    pub style: Style,
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
