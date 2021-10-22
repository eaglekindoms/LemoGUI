use winit::event::WindowEvent;
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::window::Window;

use crate::graphic::base::*;
use crate::graphic::style::Style;
use crate::widget::{Component, EventType, GEvent, State};

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
        self.message_channel.send_event(message).ok();
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

impl<'a, M: 'static> ELContext<'a, M> {
    /// 事件监听器
    /// 作用：监听用户交互事件
    pub fn component_listener(&self, listener: &mut Component<M>) -> bool
    {
        let mut key_listener = false;
        let mut mouse_listener = false;
        let hover_listener;
        let g_event = self.get_event();
        match g_event.event {
            EventType::Mouse(mouse) => {
                if g_event.state == State::Released {
                    self.window.set_ime_position(self.cursor_pos);
                }
                mouse_listener = listener.widget.action_listener(&self, mouse);
            }
            EventType::KeyBoard(key_code) => {
                key_listener = listener.widget.key_listener(&self, key_code);
            }
            EventType::ReceivedCharacter(c) => {
                listener.widget.received_character(&self, c);
            }
            _ => {}
        }
        hover_listener = listener.widget.hover_listener(&self);
        key_listener || mouse_listener || hover_listener
    }
}