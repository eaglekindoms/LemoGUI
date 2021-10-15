use winit::event::*;
use winit::event_loop::EventLoopProxy;

use crate::device::ELContext;
use crate::graphic::render_middle::RenderUtil;
use crate::graphic::style::Style;
use crate::widget::Component;
use crate::widget::message::{EventType, State};

/// 组件模型trait
/// 作用：定义组件必须的公共方法接口
pub trait ComponentModel<M> {
    /// 组件绘制方法实现
    fn draw(&self, render_utils: &mut RenderUtil);
    /// 键盘事件监听器
    fn key_listener(&mut self,
                    _action_state: ElementState,
                    _el_context: &ELContext<'_, M>,
                    _virtual_keycode: Option<VirtualKeyCode>) -> bool {
        false
    }
    /// 鼠标点击事件监听器
    fn action_listener(&mut self,
                       _action_state: ElementState,
                       _el_context: &ELContext<'_, M>) -> bool
    { false }
    /// 鼠标悬停事件监听器
    fn hover_listener(&mut self,
                      _el_context: &ELContext<'_, M>) -> bool
    { false }
}


/// 事件监听器
/// 作用：监听用户交互事件
pub fn component_listener<M>(listener: &mut Component<M>,
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
            input = listener.widget.key_listener(*state, el_context, *virtual_keycode);
        }
        WindowEvent::MouseInput {
            state,
            button: MouseButton::Left,
            ..
        }
        => {
            input = el_context.cursor_pos.is_some() &&
                listener.widget.action_listener(*state, el_context);
        }
        _ => {}
    }

    let hover = listener.widget.hover_listener(el_context);
    input || hover
}

/// 键鼠单击时，更新组件状态
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