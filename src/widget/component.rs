use crate::device::wgpu_context::WGContext;
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::RenderUtil;
use crate::widget::listener::Listener;

/// 组件模型trait
/// 作用：定义组件必须的公共方法接口
pub trait ComponentModel<M>: Listener<M> {
    /// 组件绘制方法实现
    fn draw(&mut self, wgcontext: &WGContext, render_utils: &mut RenderUtil, glob_pipeline: &PipelineState);
}
