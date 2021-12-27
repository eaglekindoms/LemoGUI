use crate::device::event_context::EventContext;
use crate::graphic::render_api::PaintBrush;

/// 渲染容器trait
/// 在事件循环时会调用实现该trait的对象
/// 作用：定义渲染所需的公共接口
pub trait Container<M>: Sized {
    /// 键鼠输入事件响应
    fn update(&mut self, event_context: &mut EventContext<'_, M>) -> bool;
    /// 容器渲染
    fn render(&mut self, paint_brush: &mut dyn PaintBrush);
}


