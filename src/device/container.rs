use winit::event::WindowEvent;

use crate::device::display_window::WGContext;
use crate::widget::component::ComponentModel;
use crate::widget::listener::Listener;

/// 渲染容器trait
/// 在事件循环时会调用实现该trait的对象
/// 作用：定义渲染所需的公共接口
pub trait Container: Sized {
    /// 通过提供的图形上下文结构体进行实例化
    fn new(wgcontext: WGContext) -> Self;
    /// 添加子组件
    fn add_comp<C>(&mut self, comp: C)
        where C: ComponentModel + Listener + 'static
    ;
    // fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    /// 事件响应
    fn input(&mut self, event: &WindowEvent) -> bool;
    /// 状态更新
    fn update(&mut self){}
    /// 容器渲染
    fn render(&mut self);
}


