use std::fmt::Debug;
use std::option::Option::Some;

use crate::device::EventContext;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::{BindEvent, Component, ComponentModel, KeyCode, Mouse};

/// 按钮控件结构体
#[derive(Debug)]
pub struct Button<M: Clone> {
    /// 组件尺寸
    pub size: Rectangle,
    /// 组件样式
    pub style: Style,
    /// 内容文本
    pub text: String,
    /// 控件状态
    pub bind_event: BindEvent<M>,
}


impl<'a, M: Clone + PartialEq> Button<M> {
    pub fn new_with_style<S: Into<String>>(rect: Rectangle, style: Style, text: S) -> Self {
        log::info!("create the Button obj use new");
        Self {
            size: rect,
            text: text.into(),
            bind_event: BindEvent::default(),
            style,
        }
    }

    pub fn new<S: Into<String>>(pos: Point<f32>, text: S) -> Self {
        let text = text.into();
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32 + 10, 40);
        log::info!("create the Button obj use default");
        Self {
            size: rect,
            style: Style::default(),
            text,
            bind_event: BindEvent::default(),
        }
    }

    /// 更新状态
    pub fn action(mut self, message: M) -> Self {
        self.bind_event.message = Some(message);
        self
    }
}

impl<M: Clone + PartialEq + 'static> From<Button<M>> for Component<M> {
    fn from(button: Button<M>) -> Self {
        Component::new(button)
    }
}

impl<'a, M: Clone + PartialEq> ComponentModel<M> for Button<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        let shape: Box<dyn ShapeGraph> = Box::new(self.size);
        paint_brush.draw_shape(&shape, self.style);
        paint_brush.draw_text(font_map, &self.size, self.text.as_str(), self.style.get_font_color());
    }
    fn key_listener(&mut self,
                    _event_context: &EventContext<'_, M>,
                    _virtual_keycode: Option<KeyCode>) -> bool {
        if let Some(key_codes) = &self.bind_event.shortcuts {
            if let Some(key) = _virtual_keycode {
                return key_codes.contains(&key);
            }
        }
        false
    }
    fn action_listener(&mut self,
                       event_context: &EventContext<'_, M>,
                       mouse: Mouse) -> bool
    {
        if mouse == self.bind_event.mouse {
            return event_context.action_animation(&mut self.style, &self.size, self.bind_event.message.clone());
        }
        false
    }
}