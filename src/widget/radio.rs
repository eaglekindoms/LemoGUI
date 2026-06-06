use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

/// 单选按钮组控件结构体
pub struct RadioGroup<M: Clone> {
    /// 各选项的圆形区域和文本标签
    pub options: Vec<(Rectangle, Label)>,
    /// 当前选中项索引
    pub selected: usize,
    /// 选中项变化回调
    pub on_change: Box<dyn Fn(usize) -> M>,
}

impl<M: Clone + PartialEq> RadioGroup<M> {
    pub fn new<S, I, F>(pos: Point<f32>, options: I, on_change: F) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
        F: 'static + Fn(usize) -> M,
    {
        let mut option_rects = Vec::new();
        let mut labels = Vec::new();
        let mut y_offset = 0.0f32;
        let radio_size = 20u32;
        let spacing = 30.0f32;
        for text in options {
            let text = text.into();
            let circle_rect = Rectangle::new(pos.x, pos.y + y_offset, radio_size, radio_size);
            let text_rect = Rectangle::new(
                pos.x + radio_size as f32 + 5.0,
                pos.y + y_offset,
                (text.len() * 10) as u32 + 10,
                radio_size,
            );
            labels.push(Label::new_text_label(text_rect, Style::default(), text));
            option_rects.push(circle_rect);
            y_offset += spacing;
        }
        Self {
            options: option_rects.into_iter().zip(labels).collect(),
            selected: 0,
            on_change: Box::new(on_change),
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }
}

impl<M: Clone + PartialEq + 'static> From<RadioGroup<M>> for Component<M> {
    fn from(rg: RadioGroup<M>) -> Self {
        Component::new(rg)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for RadioGroup<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        for (i, (rect, label)) in self.options.iter().enumerate() {
            let circle = Circle::new(
                rect.position.x + rect.width as f32 / 2.0,
                rect.position.y + rect.height as f32 / 2.0,
                rect.width as f32 / 2.0,
            );
            let outer_shape: Box<dyn ShapeGraph> = Box::new(circle);
            paint_brush.draw_shape(
                &outer_shape,
                Style::default().back_color(WHITE).border(BLACK),
            );
            if i == self.selected {
                let inner = Circle::new(
                    rect.position.x + rect.width as f32 / 2.0,
                    rect.position.y + rect.height as f32 / 2.0,
                    rect.width as f32 / 4.0,
                );
                let inner_shape: Box<dyn ShapeGraph> = Box::new(inner);
                paint_brush.draw_shape(
                    &inner_shape,
                    Style::default().back_color(LIGHT_BLUE).no_border(),
                );
            }
            label.draw(paint_brush, font_map);
        }
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let cursor = event_context.get_cursor_pos();
        if let EventType::Mouse(Mouse::Left) = g_event.event {
            if g_event.state == State::Pressed {
                for (i, (rect, label)) in self.options.iter().enumerate() {
                    if (rect.contain_coord(cursor) || label.size.contain_coord(cursor))
                        && i != self.selected
                    {
                        self.selected = i;
                        event_context.send_message((self.on_change)(i));
                        return true;
                    }
                }
            }
        }
        self.options
            .iter()
            .any(|(rect, label)| rect.contain_coord(cursor) || label.size.contain_coord(cursor))
    }
}
