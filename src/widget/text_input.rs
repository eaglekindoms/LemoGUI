use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

/// 按钮控件结构体
#[allow(missing_debug_implementations)]
pub struct TextInput<M: Clone> {
    /// 组件面板
    pub text_label: Label,
    /// 控件状态
    pub state: Option<M>,
    pub text_receive: Box<dyn Fn(String) -> M>,
    ///是否聚焦
    pub is_focus: bool,
}

impl<'a, M: Clone + PartialEq> TextInput<M> {
    pub fn new_with_style<S: Into<String>, MT>(
        rect: Rectangle,
        style: Style,
        text: S,
        rec: MT,
    ) -> Self
    where
        MT: 'static + Fn(String) -> M,
    {
        Self {
            text_label: Label::new_text_label(rect, style, text.into()),
            state: None,
            is_focus: false,
            text_receive: Box::new(rec),
        }
    }

    pub fn new<S: Into<String>, MT>(pos: Point<f32>, text: S, rec: MT) -> Self
    where
        MT: 'static + Fn(String) -> M,
    {
        let text = text.into();
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32 + 10, 40);
        let style = Style::default().back_color(WHITE);
        Self {
            text_label: Label::new_text_label(rect, style, text),
            state: None,
            text_receive: Box::new(rec),
            is_focus: false,
        }
    }

    pub fn focused(mut self, focus: bool) -> Self {
        self.is_focus = focus;
        self
    }

    fn received_character(&mut self, event_context: &mut dyn EventContext<M>, c: char) -> bool {
        if !self.is_focus {
            return false;
        }
        if c == '\u{8}' || c == '\u{7f}' {
            if let Some(text) = &mut self.text_label.text {
                text.pop();
                event_context.send_message((self.text_receive)(text.clone()));
            }
            return true;
        }
        if c == '\r' || c.is_control() {
            return true;
        }
        if let Some(text) = &mut self.text_label.text {
            text.push(c);
            event_context.send_message((self.text_receive)(text.clone()));
        }
        true
    }
}

impl<M: Clone + PartialEq + 'static> From<TextInput<M>> for Component<M> {
    fn from(text_input: TextInput<M>) -> Self {
        Component::new(text_input)
    }
}

impl<'a, M: Clone + PartialEq> ComponentModel<M> for TextInput<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        self.text_label.draw(paint_brush, font_map)
    }
    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let hover = self
            .text_label
            .size
            .contain_coord(event_context.get_cursor_pos());
        if hover {
            event_context.set_cursor_icon(Cursor::Text);
        }
        match g_event.event {
            EventType::Mouse(Mouse::Left) if g_event.state == State::Pressed => {
                if hover {
                    self.is_focus = true;
                    let r = self.text_label.size;
                    event_context.set_ime_position(
                        Point::new(r.position.x + 4.0, r.position.y + 4.0),
                        r.height as f32,
                    );
                    if let Some(text) = &self.text_label.text {
                        event_context.send_message((self.text_receive)(text.clone()));
                    }
                    return true;
                }
                if self.is_focus {
                    self.is_focus = false;
                }
                false
            }
            EventType::ReceivedCharacter(c) => self.received_character(event_context, c),
            EventType::KeyBoard(Some(KeyCode::Backspace))
                if g_event.state == State::Pressed =>
            {
                self.received_character(event_context, '\u{8}')
            }
            _ => false,
        }
    }

    fn ime_caret(&self) -> Option<(Point<f32>, f32)> {
        if !self.is_focus {
            return None;
        }
        let r = self.text_label.size;
        Some((
            Point::new(r.position.x + 4.0, r.position.y + 4.0),
            r.height as f32,
        ))
    }
}
