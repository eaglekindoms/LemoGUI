use crate::device::event_context::ELContext;
use crate::graphic::render_middle::RenderUtil;

/// 渲染容器trait
/// 在事件循环时会调用实现该trait的对象
/// 作用：定义渲染所需的公共接口
pub trait Container<M>: Sized {
    /// 键鼠输入事件响应
    fn update(&mut self, el_context: &mut ELContext<'_, M>) -> bool;
    /// 容器渲染
    fn render(&self, utils: &mut RenderUtil);
}


