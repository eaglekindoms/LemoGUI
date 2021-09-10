use crate::device::event_context::ELContext;
use crate::device::wgpu_context::WGContext;
use crate::widget::ComponentModel;
use crate::widget::Instance;

/// 渲染容器trait
/// 在事件循环时会调用实现该trait的对象
/// 作用：定义渲染所需的公共接口
pub trait Container<M>: Sized {
    /// 通过提供的图形上下文结构体进行实例化
    fn new(wgcontext: &WGContext) -> Self;
    /// 添加子组件
    fn add_comp(&mut self, comp: &impl Instance<M=M>);
    // fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    /// 键鼠输入事件响应
    fn input(&mut self,
             wgcontext: &mut WGContext,
             el_context: &mut ELContext<'_, M>,
             instance: &mut impl Instance<M=M>) -> bool;
    /// 状态更新
    fn update(&mut self) {}
    /// 容器渲染
    fn render(&mut self, wgcontext: &mut WGContext);
}


