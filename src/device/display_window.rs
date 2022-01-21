use std::fmt::Debug;

use crate::device::container::Container;
use crate::device::GPUContext;
use crate::device::{EventContext, EventListener};
use crate::widget::Setting;

/// 窗口结构体
/// 作用：封装窗体，事件循环器，图形上下文
pub struct DisplayWindow<'a, M: 'static> {
    /// 图形上下文
    pub gpu_context: GPUContext,
    /// 时间监听器
    pub(crate) event_loop: EventListener<M>,
    /// 事件上下文
    pub(crate) event_context: EventContext<'a, M>,
}

impl<M: 'static + Debug> DisplayWindow<'static, M> {
    pub fn start<C>(self, container: C)
    where
        C: Container<M> + 'static,
    {
        crate::device::run_instance(self, container);
    }

    pub fn new<'a>(setting: Setting) -> DisplayWindow<'a, M> {
        crate::device::init_window(setting)
    }
}
