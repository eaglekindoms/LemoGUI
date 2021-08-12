use std::fmt::Debug;

use winit::event::*;
use winit::event_loop::EventLoopProxy;

use crate::device::event_context::ELContext;
use crate::graphic::base::shape::Point;
use crate::widget::message::Message;

/// 事件监听器
/// 作用：监听用户交互事件
pub trait Listener<M> {
    fn listener(&mut self, el_context: &ELContext<'_, M>) -> bool
    {
        let mut input = false;
        match el_context.window_event.as_ref().unwrap() {
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
                input = self.action_listener(*state, el_context);
            }
            _ => {}
        }
        input = self.custom_listener(el_context);
        input
    }
    /// 键盘事件监听器
    fn key_listener(&mut self, virtual_keycode: Option<VirtualKeyCode>) -> bool {
        false
    }

    /// 鼠标事件监听器
    fn action_listener(&mut self, state: ElementState,
                       el_context: &ELContext<'_, M>) -> bool
    { false }

    fn custom_listener(&mut self, el_context: &ELContext<'_, M>) -> bool {
        let mut input = false;
        if let Some(broadcast) = el_context.custom_event.as_ref() {
            input = self.sub_listener(broadcast);
        }
        input
    }

    fn sub_listener(&mut self, broadcast: &M) -> bool { false }
}