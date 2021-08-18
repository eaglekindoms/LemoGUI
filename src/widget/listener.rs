use winit::event::*;
use winit::event_loop::EventLoopProxy;

use crate::device::event_context::ELContext;
use crate::graphic::style::Style;
use crate::widget::component::ComponentModel;
use crate::widget::message::{EventType, State};

/// 事件监听器
/// 作用：监听用户交互事件
pub trait Listener<M> {
    /// 键盘事件监听器
    fn key_listener(&mut self, _action_state: ElementState, _el_context: &ELContext<'_, M>, _virtual_keycode: Option<VirtualKeyCode>) -> bool {
        false
    }

    /// 鼠标事件监听器
    fn action_listener(&mut self, _action_state: ElementState,
                       _el_context: &ELContext<'_, M>) -> bool
    { false }
    fn hover_listener(&mut self,
                      _el_context: &ELContext<'_, M>) -> bool
    { false }
    // 组件消息监听器
    fn message_listener(&mut self, _broadcast: &M) -> bool { false }
}

pub fn component_listener<M>(listener: &mut Box<dyn ComponentModel<M>>,
                             el_context: &ELContext<'_, M>) -> bool
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
            input = listener.key_listener(*state, el_context, *virtual_keycode);
        }
        WindowEvent::MouseInput {
            state,
            button: MouseButton::Left,
            ..
        }
        => {
            input = el_context.cursor_pos.is_some() &&
                listener.action_listener(*state, el_context);
        }
        _ => {}
    }
    let mut custom = false;
    if let Some(broadcast) = el_context.message.as_ref() {
        custom = listener.message_listener(broadcast);
    }

    let hover = listener.hover_listener(el_context);
    input || custom || hover
}

pub fn action_animation<M: Copy>(style: &mut Style,
                                 action_state: ElementState,
                                 event_loop_proxy: &EventLoopProxy<M>,
                                 com_state: &Option<State<M>>,
                                 virtual_keycode: Option<VirtualKeyCode>,
) -> bool
{
    if com_state.is_some() {
        let hover_color = style.get_hover_color();
        let back_color = style.get_back_color();
        if let Some(state) = com_state {
            if (state.event == EventType::Mouse)
                || (virtual_keycode.is_some()
                && state.event == EventType::KeyBoard(virtual_keycode.unwrap())) {
                if action_state == ElementState::Pressed {
                    style.display_color(hover_color);
                    if let Some(message) = state.message {
                        event_loop_proxy.send_event(message).ok();
                    }
                } else if action_state == ElementState::Released {
                    style.display_color(back_color);
                }
                return true;
            }
        }
    }
    return false;
}