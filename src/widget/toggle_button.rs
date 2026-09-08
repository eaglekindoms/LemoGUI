use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

/// 粘滞按下按钮（工具栏 B/I/U）
pub struct ToggleButton<M: Clone> {
    pub button_label: Label,
    pub pressed: bool,
    pub on_change: Box<dyn Fn(bool) -> M>,
    hover: bool,
    armed: bool,
}

impl<M: Clone + PartialEq> ToggleButton<M> {
    pub fn new<S: Into<String>, F>(pos: Point<f32>, text: S, on_change: F) -> Self
    where
        F: 'static + Fn(bool) -> M,
    {
        let text = text.into();
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 12) as u32 + 16, 32);
        Self::new_with_rect(rect, text, on_change)
    }

    pub fn new_with_rect<S: Into<String>, F>(rect: Rectangle, text: S, on_change: F) -> Self
    where
        F: 'static + Fn(bool) -> M,
    {
        Self {
            button_label: Label::new_text_label(rect, Style::default().border(BLACK), text.into()),
            pressed: false,
            on_change: Box::new(on_change),
            hover: false,
            armed: false,
        }
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }
}

impl<M: Clone + PartialEq + 'static> From<ToggleButton<M>> for Component<M> {
    fn from(tb: ToggleButton<M>) -> Self {
        Component::new(tb)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for ToggleButton<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        let idle = if self.pressed { LIGHT_BLUE } else { LIGHT_WHITE };
        let hover = LIGHT_BLUE;
        let fill = if self.armed && self.hover {
            hover.darken(0.75)
        } else if self.hover {
            if self.pressed {
                hover.darken(0.85)
            } else {
                hover
            }
        } else {
            idle
        };
        let label = Label::new_text_label(
            self.button_label.size,
            Style::default()
                .back_color(fill)
                .border(BLACK)
                .font_color(BLACK),
            self.button_label.text.clone().unwrap_or_default(),
        );
        label.draw(paint_brush, font_map);
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let hover = self
            .button_label
            .size
            .contain_coord(event_context.get_cursor_pos());
        let prev_hover = self.hover;
        let prev_armed = self.armed;
        self.hover = hover;
        sync_armed(event_context, hover, &mut self.armed);
        let visual = self.hover != prev_hover || self.armed != prev_armed;
        if let EventType::Mouse(Mouse::Left) = g_event.event {
            if g_event.state == State::Pressed && hover {
                self.pressed = !self.pressed;
                event_context.send_message((self.on_change)(self.pressed));
                return true;
            }
        }
        visual
    }
}
