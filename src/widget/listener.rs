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
                state,
                button: MouseButton::Left,
                ..
            }
            => {
                if let Some(pos) = cursor_pos {
                    input = self.action_listener(*state, pos);
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
    fn action_listener(&mut self, state: ElementState, position: Point<f32>) -> bool { false }
}