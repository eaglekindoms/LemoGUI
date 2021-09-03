use crate::device::wgpu_context::WGContext;
use crate::graphic::base::color::LIGHT_BLUE;
use crate::graphic::base::shape::ShapeGraph;
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::RenderUtil;
use crate::graphic::style::Style;
use crate::widget::component::ComponentModel;

/// 图形绘制面板控件结构体
pub struct ShapeBoard {
    pub shape_arr: Vec<Box<dyn ShapeGraph>>,
    pub style: Style,
}

impl<M> ComponentModel<M> for ShapeBoard {
    fn draw(&self, wgcontext: &WGContext, render_utils: &mut RenderUtil, glob_pipeline: &PipelineState) {
        let mut style = self.style;
        for shape in &self.shape_arr {
            shape.to_buffer(wgcontext, style.get_back_color())
                .render(render_utils, &glob_pipeline, shape.get_type());
            style = Style::default().back_color(LIGHT_BLUE).round();
        }
    }
}
