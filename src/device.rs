/// 图形渲染采用wgpu实现
#[cfg(feature = "wgpu_impl")]
pub type GPUContext = crate::backend::wgpu_impl::WGPUContext;
#[cfg(feature = "wgpu_impl")]
pub type VBuffer = crate::backend::wgpu_impl::VertexBuffer;
/// 窗口及事件监听器采用winit实现
#[cfg(feature = "winit_impl")]
pub type EventContext<M> = crate::backend::winit_impl::WEventContext<M>;
#[cfg(feature = "winit_impl")]
pub type EventListener<M> = winit::event_loop::EventLoop<M>;
/// 窗口及事件监听器采用sdl2实现
#[cfg(feature = "sdl2_impl")]
pub type EventContext<M> = crate::backend::sdl2_impl::SEventContext<M>;
#[cfg(feature = "sdl2_impl")]
pub type EventListener<M> = crate::backend::sdl2_impl::SEventListener<M>;

/// 窗口结构体
/// 作用：封装窗体，事件循环器，图形上下文
pub struct DisplayWindow<M: 'static> {
    /// 图形上下文
    pub gpu_context: GPUContext,
    /// 时间监听器
    pub(crate) event_loop: EventListener<M>,
    /// 事件上下文
    pub(crate) event_context: EventContext<M>,
    /// 字体缓冲
    pub font_map: crate::graphic::base::GCharMap,
}

impl<M: 'static + std::fmt::Debug> DisplayWindow<M> {
    pub fn start<C>(self, container: C)
    where
        C: crate::widget::ComponentModel<M> + 'static,
    {
        run_instance(self, container);
    }

    pub fn new<'a>(setting: crate::widget::Setting) -> DisplayWindow<M> {
        init_window(setting)
    }
}

/// 窗口启动方法
fn run_instance<C, M>(window: DisplayWindow<M>, container: C)
where
    C: crate::widget::ComponentModel<M> + 'static,
    M: 'static + std::fmt::Debug,
{
    #[cfg(feature = "winit_impl")]
    crate::backend::winit_impl::run(window, container);

    #[cfg(feature = "sdl2_impl")]
    crate::backend::sdl2_impl::run(window, container);
}

/// 基于winit的初始化窗口方法
fn init_window<M: 'static + std::fmt::Debug>(setting: crate::widget::Setting) -> DisplayWindow<M> {
    use futures::executor::block_on;
    #[cfg(feature = "winit_impl")]
    return block_on(crate::backend::winit_impl::init(setting));
    #[cfg(feature = "sdl2_impl")]
    return block_on(crate::backend::sdl2_impl::init(setting));
}
