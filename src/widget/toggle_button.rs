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
        let label = Label::new_text_label(
            self.button_label.size,
            if self.pressed {
                Style::default()
                    .back_color(LIGHT_BLUE)
                    .border(BLACK)
                    .font_color(BLACK)
            } else {
                Style::default()
                    .back_color(LIGHT_WHITE)
                    .border(BLACK)
                    .font_color(BLACK)
            },
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
        if let EventType::Mouse(Mouse::Left) = g_event.event {
            if g_event.state == State::Pressed && hover {
                self.pressed = !self.pressed;
                event_context.send_message((self.on_change)(self.pressed));
                return true;
            }
        }
        false
    }
}
