use crate::device::event_context::ELContext;
use crate::device::wgpu_context::WGContext;
use crate::widget::component::ComponentModel;

/// 渲染容器trait
/// 在事件循环时会调用实现该trait的对象
/// 作用：定义渲染所需的公共接口
pub trait Container<M>: Sized {
    /// 通过提供的图形上下文结构体进行实例化
    fn new(wgcontext: WGContext) -> Self;
    /// 添加子组件
    fn add_comp<C>(&mut self, comp: C)
        where C: ComponentModel<M> + 'static;
    // fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    /// 键鼠输入事件响应
    fn input(&mut self, el_context: &mut ELContext<'_, M>) -> bool;
    /// 状态更新
    fn update(&mut self) {}
    /// 容器渲染
    fn render(&mut self);
}


