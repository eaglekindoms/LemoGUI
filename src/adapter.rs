/// 图形渲染采用wgpu实现
#[cfg(feature = "wgpu_impl")]
mod wgpu_adapter {
    pub type GPUContext = crate::backend::wgpu_impl::WGPUContext;
    pub type VBuffer = crate::backend::wgpu_impl::VertexBuffer;
    pub type TextureBuffer = crate::backend::wgpu_impl::TextureBufferData;
}
#[cfg(feature = "wgpu_impl")]
pub use wgpu_adapter::*;

/// 图形渲染采用wgpu实现
#[cfg(feature = "glow_impl")]
mod glow_adapter {
    pub type GPUContext = crate::backend::glow_impl::GLGPUContext;
}
#[cfg(feature = "glow_impl")]
#[cfg(not(feature = "wgpu_impl"))]
pub use glow_adapter::*;

/// 窗口结构体
/// 作用：封装窗体，事件循环器，图形上下文
#[cfg(feature = "sdl2_impl")]
#[cfg(not(feature = "winit_impl"))]
pub struct DisplayWindow<M: 'static> {
    /// 图形上下文
    pub gpu_context: GPUContext,
    /// 时间监听器
    pub(crate) event_loop: sdl2::EventPump,
    /// 事件上下文
    pub(crate) event_context: crate::backend::sdl2_impl::SEventContext<M>,
    /// 字体缓冲
    pub font_map: crate::graphic::base::GCharMap,
}

#[cfg(feature = "winit_impl")]
pub struct DisplayWindow<M: 'static> {
    /// 图形上下文
    pub gpu_context: GPUContext,
    /// 时间监听器
    pub(crate) event_loop: winit::event_loop::EventLoop<M>,
    /// 事件上下文
    pub(crate) event_context: crate::backend::winit_impl::WEventContext<M>,
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

    pub fn new<'a>(setting: crate::instance::Setting) -> DisplayWindow<M> {
        init_window(setting)
    }
}

/// 窗口启动方法
fn run_instance<C, M>(window: DisplayWindow<M>, container: C)
where
    C: crate::widget::ComponentModel<M> + 'static,
    M: 'static + std::fmt::Debug,
{
    // 使用winit进行事件监听
    #[cfg(feature = "winit_impl")]
    return crate::backend::winit_impl::run(window, container);
    // 使用sdl2进行事件监听
    #[cfg(feature = "sdl2_impl")]
    return crate::backend::sdl2_impl::run(window, container);
}

/// 初始化窗口方法
fn init_window<M: 'static + std::fmt::Debug>(
    setting: crate::instance::Setting,
) -> DisplayWindow<M> {
    use futures::executor::block_on;
    // 使用winit初始化窗口
    #[cfg(feature = "winit_impl")]
    return block_on(crate::backend::winit_impl::init(setting));
    // 使用sdl2初始化窗口
    #[cfg(feature = "sdl2_impl")]
    return block_on(crate::backend::sdl2_impl::init(setting));
}
