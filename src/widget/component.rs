use crate::device::ELContext;
use crate::graphic::render_middle::RenderUtil;
use crate::widget::{Component, Mouse, State};
use crate::widget::event::EventType;

/// 组件模型trait
/// 作用：定义组件必须的公共方法接口
pub trait ComponentModel<M> {
    /// 组件绘制方法实现
    fn draw(&self, render_utils: &mut RenderUtil);
    /// 键盘事件监听器
    fn key_listener(&mut self,
                    _el_context: &ELContext<'_, M>) -> bool {
        false
    }
    /// 鼠标点击事件监听器
    fn action_listener(&mut self,
                       _el_context: &ELContext<'_, M>) -> bool
    { false }
    /// 鼠标悬停事件监听器
    fn hover_listener(&mut self,
                      _el_context: &ELContext<'_, M>) -> bool
    { false }
    fn received_character(&mut self,
                          _el_context: &ELContext<'_, M>, c: char) -> bool
    { false }
}


/// 事件监听器
/// 作用：监听用户交互事件
pub fn component_listener<M>(listener: &mut Component<M>,
                             el_context: &ELContext<'_, M>) -> bool
{
    let mut key_listener = false;
    let mut mouse_listener = false;
    let hover_listener;
    let g_event = el_context.get_event();
    match g_event.event {
        EventType::Mouse(mouse) => {
            if g_event.state == State::Released {
                el_context.window.set_ime_position(el_context.cursor_pos);
            }
            if mouse == Mouse::Left {
                mouse_listener = listener.widget.action_listener(el_context);
            }
        }
        EventType::KeyBoard(_) => {
            key_listener = listener.widget.key_listener(el_context);
        }
        EventType::ReceivedCharacter(c) => {
            listener.widget.received_character(el_context, c);
        }
        _ => {}
    }
    hover_listener = listener.widget.hover_listener(el_context);
    key_listener || mouse_listener || hover_listener
}
