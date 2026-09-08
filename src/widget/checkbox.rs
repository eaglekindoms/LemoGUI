use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

/// 复选框控件结构体
pub struct Checkbox<M: Clone> {
    /// 勾选框区域
    pub box_rect: Rectangle,
    /// 文本标签
    pub text_label: Label,
    /// 是否选中
    pub checked: bool,
    /// 状态变化回调
    pub on_change: Box<dyn Fn(bool) -> M>,
    hover: bool,
    armed: bool,
}

impl<M: Clone + PartialEq> Checkbox<M> {
    pub fn new<S: Into<String>, F>(pos: Point<f32>, text: S, on_change: F) -> Self
    where
        F: 'static + Fn(bool) -> M,
    {
        let text = text.into();
        let box_size = 20u32;
        let box_rect = Rectangle::new(pos.x, pos.y, box_size, box_size);
        let text_rect = Rectangle::new(
            pos.x + box_size as f32 + 5.0,
            pos.y,
            (text.len() * 10) as u32 + 10,
            box_size,
        );
        Self {
            box_rect,
            text_label: Label::new_text_label(text_rect, Style::default(), text),
            checked: false,
            on_change: Box::new(on_change),
            hover: false,
            armed: false,
        }
    }

    pub fn new_with_style<S: Into<String>, F>(
        box_rect: Rectangle,
        text_rect: Rectangle,
        style: Style,
        text: S,
        on_change: F,
    ) -> Self
    where
        F: 'static + Fn(bool) -> M,
    {
        Self {
            box_rect,
            text_label: Label::new_text_label(text_rect, style, text.into()),
            checked: false,
            on_change: Box::new(on_change),
            hover: false,
            armed: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    fn is_hover(&self, cursor: Point<f32>) -> bool {
        self.box_rect.contain_coord(cursor) || self.text_label.size.contain_coord(cursor)
    }
}

impl<M: Clone + PartialEq + 'static> From<Checkbox<M>> for Component<M> {
    fn from(cb: Checkbox<M>) -> Self {
        Component::new(cb)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for Checkbox<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        let fill = pointer_fill(
            &Style::default().back_color(WHITE).hover_color(LIGHT_BLUE),
            self.hover,
            self.armed,
        );
        let box_style = Style::default().border(BLACK).back_color(fill);
        let shape: Box<dyn ShapeGraph> = Box::new(self.box_rect);
        paint_brush.draw_shape(&shape, box_style);
        if self.checked {
            let inner = Rectangle::new(
                self.box_rect.position.x + 4.0,
                self.box_rect.position.y + 4.0,
                self.box_rect.width - 8,
                self.box_rect.height - 8,
            );
            let inner_shape: Box<dyn ShapeGraph> = Box::new(inner);
            paint_brush.draw_shape(&inner_shape, Style::default().back_color(LIGHT_BLUE).no_border());
        }
        self.text_label.draw(paint_brush, font_map);
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let hover = self.is_hover(event_context.get_cursor_pos());
        let prev_hover = self.hover;
        let prev_armed = self.armed;
        self.hover = hover;
        sync_armed(event_context, hover, &mut self.armed);
        let visual = self.hover != prev_hover || self.armed != prev_armed;
        if let EventType::Mouse(Mouse::Left) = g_event.event {
            if g_event.state == State::Pressed && hover {
                self.checked = !self.checked;
                event_context.send_message((self.on_change)(self.checked));
                return true;
            }
        }
        visual
    }
}
