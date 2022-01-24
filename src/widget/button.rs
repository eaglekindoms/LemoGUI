use std::fmt::Debug;
use std::option::Option::Some;

use crate::device::EventContext;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

/// 按钮控件结构体
#[derive(Debug)]
pub struct Button<M: Clone> {
    /// 组件面板
    pub button_label: Label,
    /// 控件状态
    pub bind_event: BindEvent<M>,
}

impl<'a, M: Clone + PartialEq> Button<M> {
    pub fn new_with_style<S: Into<String>>(rect: Rectangle, style: Style, text: S) -> Self {
        Self {
            button_label: Label::new_text_label(rect, style, text.into()),
            bind_event: BindEvent::default(),
        }
    }

    pub fn new<S: Into<String>>(pos: Point<f32>, text: S) -> Self {
        let text = text.into();
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32 + 10, 40);
        let style = Style::default();
        Self {
            button_label: Label::new_text_label(rect, style, text),
            bind_event: BindEvent::default(),
        }
    }

    /// 更新状态
    pub fn action(mut self, message: M) -> Self {
        self.bind_event.message = Some(message);
        self
    }
    fn key_listener(
        &mut self,
        _event_context: &EventContext<M>,
        virtual_keycode: Option<KeyCode>,
    ) -> bool {
        if let Some(key_codes) = &self.bind_event.shortcuts {
            if let Some(key) = virtual_keycode {
                return key_codes.contains(&key);
            }
        }
        false
    }
    fn action_listener(&mut self, event_context: &EventContext<M>, mouse: Mouse) -> bool {
        if mouse == self.bind_event.mouse {
            return component::action_animation(
                event_context,
                &mut self.button_label.style,
                &self.button_label.size,
                self.bind_event.message.clone(),
            );
        }
        false
    }
}

impl<M: Clone + PartialEq + 'static> From<Button<M>> for Component<M> {
    fn from(button: Button<M>) -> Self {
        Component::new(button)
    }
}

impl<'a, M: Clone + PartialEq> ComponentModel<M> for Button<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        self.button_label.draw(paint_brush, font_map)
    }
    fn listener(&mut self, event_context: &mut EventContext<M>) -> bool {
        let mut key_listener = false;
        let mut mouse_listener = false;
        let g_event = event_context.get_event();
        match g_event.event {
            EventType::Mouse(mouse) => {
                if g_event.state == State::Released {
                    event_context.set_ime_position();
                }
                mouse_listener = self.action_listener(&event_context, mouse);
            }
            EventType::KeyBoard(key_code) => {
                key_listener = self.key_listener(&event_context, key_code);
            }
            _ => {}
        }
        key_listener || mouse_listener
    }
}
