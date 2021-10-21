use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;

use crate::graphic::base::*;

/// 事件上下文
pub struct ELContext<'a, M: 'static> {
    /// 窗口id
    pub window: Window,
    /// 鼠标位置
    pub cursor_pos: Option<Point<f32>>,
    /// 窗口事件
    pub window_event: Option<WindowEvent<'a>>,
    /// 自定义事件
    pub message: Option<M>,
    /// 自定义事件广播器
    pub message_channel: EventLoopProxy<M>,
}

impl<'a, M: 'static> ELContext<'a, M> {
    // 更新鼠标坐标
    pub fn update_cursor<P: Into<Point<f32>>>(&mut self, pos: P) {
        self.cursor_pos = Some(pos.into());
    }
}

impl From<Point<f32>> for winit::dpi::Position {
    #[inline]
    fn from(position: Point<f32>) -> winit::dpi::Position {
        winit::dpi::Position::Physical(
            winit::dpi::PhysicalPosition
            {
                x: position.x as i32,
                y: position.y as i32,
            })
    }
}

impl From<winit::dpi::PhysicalPosition<f64>> for Point<f32> {
    #[inline]
    fn from(position: winit::dpi::PhysicalPosition<f64>) -> Point<f32> {
        Point::new(position.x as f32, position.y as f32)
    }
}

impl From<winit::dpi::PhysicalSize<u32>> for Point<u32> {
    fn from(position: PhysicalSize<u32>) -> Self {
        Point::new(position.width, position.height)
    }
}