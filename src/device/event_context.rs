use winit::event::WindowEvent;
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::window::Window;

use crate::graphic::base::*;
use crate::graphic::style::Style;
use crate::widget::{GEvent, State};

/// 事件上下文
pub struct ELContext<'a, M: 'static> {
    /// 窗口id
    pub window: Window,
    /// 鼠标位置
    pub cursor_pos: Point<f32>,
    /// 窗口事件
    pub window_event: Option<WindowEvent<'a>>,
    /// 自定义事件
    pub message: Option<M>,
    /// 自定义事件广播器
    message_channel: EventLoopProxy<M>,
}

impl<'a, M: 'static> ELContext<'a, M> {
    pub fn new(window: Window, event_loop: &EventLoop<M>) -> ELContext<'a, M> {
        ELContext {
            window,
            cursor_pos: Point::new(-1.0, -1.0),
            window_event: None,
            message: None,
            message_channel: event_loop.create_proxy(),
        }
    }

    // 更新鼠标坐标
    pub fn update_cursor<P: Into<Point<f32>>>(&mut self, pos: P) {
        self.cursor_pos = pos.into();
    }

    pub fn get_event(&self) -> GEvent {
        self.window_event.as_ref().unwrap().into()
    }

    pub fn send_message(&self, message: M) {
        self.message_channel.send_event(message);
    }
    /// 键鼠单击动画效果
    pub fn action_animation(&self, style: &mut Style, position: &Rectangle,
                            message: Option<M>) -> bool {
        let input = position
            .contain_coord(self.cursor_pos);
        if input && message.is_some() {
            let message = message.unwrap();
            let hover_color = style.get_hover_color();
            let back_color = style.get_back_color();
            if self.get_event().state == State::Pressed {
                style.display_color(hover_color);
                self.send_message(message);
            } else if self.get_event().state == State::Released {
                style.display_color(back_color);
            }
            return true;
        }
        return false;
    }
}