pub use container::*;
pub use display_window::*;

mod container;
mod display_window;

/// 图形渲染采用wgpu实现
pub type GPUContext = crate::backend::wgpu_impl::WGPUContext;
pub type VBuffer = crate::backend::wgpu_impl::VertexBuffer;
/// 窗口及事件监听器采用winit实现
pub type EventContext<'a, M> = crate::backend::winit_impl::WEventContext<'a, M>;
pub type EventListener<M> = winit::event_loop::EventLoop<M>;

/// 基于winit的窗口启动方法
fn run_instance<C, M>(window: DisplayWindow<'static, M>, container: C)
where
    C: Container<M> + 'static,
    M: 'static + std::fmt::Debug,
{
    crate::backend::winit_impl::run(window, container)
}

/// 基于winit的初始化窗口方法
fn init_window<'a, M: 'static + std::fmt::Debug>(
    setting: crate::widget::Setting,
) -> DisplayWindow<'a, M> {
    use futures::executor::block_on;
    block_on(crate::backend::winit_impl::init(setting))
}
