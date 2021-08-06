use std::fmt::{Debug, Formatter};

use winit::event::*;

use crate::graphic::base::shape::Point;

/// 事件监听器
/// 作用：监听用户交互事件
pub trait Listener {
    fn listener(&mut self, cursor_pos: Option<Point<f32>>, event: &WindowEvent) -> bool {
        let mut input = false;
        match event {
            WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state,
                    virtual_keycode,
                    ..
                },
                ..
            } => {
                if *state == ElementState::Released {
                    input = self.key_listener(*virtual_keycode);
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            }
            => {
                match cursor_pos {
                    None => {}
                    Some(pos) => {
                        input = self.action_listener(pos);
                    }
                }
            }
            _ => {}
        }
        input
    }
    /// 键盘事件监听器
    fn key_listener(&mut self, virtual_keycode: Option<VirtualKeyCode>) -> bool {
        false
    }

    /// 鼠标事件监听器
    fn action_listener(&mut self, position: Point<f32>) -> bool { false }
}