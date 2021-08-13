use winit::event::WindowEvent;
use winit::event_loop::EventLoopProxy;
use winit::window::WindowId;

use crate::graphic::base::shape::Point;

/// 事件上下文
pub struct ELContext<'a, M: 'static> {
    /// 窗口id
    pub window_id: WindowId,
    /// 鼠标位置
    pub cursor_pos: Option<Point<f32>>,
    /// 窗口事件
    pub window_event: Option<WindowEvent<'a>>,
    /// 自定义事件
    pub message: Option<M>,
    /// 自定义事件广播器
    pub message_channel: EventLoopProxy<M>,
}