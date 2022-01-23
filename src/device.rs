#[cfg(feature = "sdl2_impl")]
use crate::backend::sdl2_impl;
#[cfg(feature = "winit_impl")]
use crate::backend::winit_impl;

/// 图形渲染采用wgpu实现
#[cfg(feature = "wgpu_impl")]
pub type GPUContext = crate::backend::wgpu_impl::WGPUContext;
#[cfg(feature = "wgpu_impl")]
pub type VBuffer = crate::backend::wgpu_impl::VertexBuffer;
/// 窗口及事件监听器采用winit实现
#[cfg(feature = "winit_impl")]
pub type EventContext<'a, M> = winit_impl::WEventContext<'a, M>;
#[cfg(feature = "winit_impl")]
pub type EventListener<M> = winit::event_loop::EventLoop<M>;
/// 窗口及事件监听器采用sdl2实现
#[cfg(feature = "sdl2_impl")]
pub type EventContext<'a, M> = sdl2_impl::WEventContext<M>;
#[cfg(feature = "sdl2_impl")]
pub type EventListener<M> = sdl2::EventPump;

/// 窗口结构体
/// 作用：封装窗体，事件循环器，图形上下文
pub struct DisplayWindow<'a, M: 'static> {
    /// 图形上下文
    pub gpu_context: GPUContext,
    /// 时间监听器
    pub(crate) event_loop: EventListener<M>,
    /// 事件上下文
    pub(crate) event_context: EventContext<'a, M>,
    /// 字体缓冲
    pub font_map: crate::graphic::base::GCharMap,
}

impl<M: 'static + std::fmt::Debug> DisplayWindow<'static, M> {
    pub fn start<C>(self, container: C)
    where
        C: crate::widget::ComponentModel<M> + 'static,
    {
        run_instance(self, container);
    }

    pub fn new<'a>(setting: crate::widget::Setting) -> DisplayWindow<'a, M> {
        init_window(setting)
    }
}

/// 基于winit的窗口启动方法
#[cfg(feature = "winit_impl")]
fn run_instance<C, M>(window: DisplayWindow<'static, M>, container: C)
where
    C: crate::widget::ComponentModel<M> + 'static,
    M: 'static + std::fmt::Debug,
{
    crate::backend::winit_impl::run(window, container)
}

/// 基于winit的初始化窗口方法
#[cfg(feature = "winit_impl")]
fn init_window<'a, M: 'static + std::fmt::Debug>(
    setting: crate::widget::Setting,
) -> DisplayWindow<'a, M> {
    use futures::executor::block_on;
    block_on(crate::backend::winit_impl::init(setting))
}
