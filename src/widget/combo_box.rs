use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

/// 下拉选择框（字号等）
pub struct ComboBox<M: Clone> {
    pub bounds: Rectangle,
    pub items: Vec<String>,
    pub selected: usize,
    pub open: bool,
    pub item_height: u32,
    pub on_select: Box<dyn Fn(usize) -> M>,
    pub on_toggle: Box<dyn Fn(bool) -> M>,
}

impl<M: Clone + PartialEq> ComboBox<M> {
    pub fn new<S, I, FS, FT>(
        bounds: Rectangle,
        items: I,
        selected: usize,
        open: bool,
        on_select: FS,
        on_toggle: FT,
    ) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
        FS: 'static + Fn(usize) -> M,
        FT: 'static + Fn(bool) -> M,
    {
        let items: Vec<String> = items.into_iter().map(|s| s.into()).collect();
        let selected = selected.min(items.len().saturating_sub(1));
        Self {
            bounds,
            items,
            selected,
            open,
            item_height: 26,
            on_select: Box::new(on_select),
            on_toggle: Box::new(on_toggle),
        }
    }

    fn dropdown_rect(&self) -> Rectangle {
        let h = self.item_height * self.items.len() as u32;
        Rectangle::new(
            self.bounds.position.x,
            self.bounds.position.y + self.bounds.height as f32,
            self.bounds.width,
            h,
        )
    }

    fn item_rect(&self, index: usize) -> Rectangle {
        let drop = self.dropdown_rect();
        Rectangle::new(
            drop.position.x,
            drop.position.y + (index as u32 * self.item_height) as f32,
            drop.width,
            self.item_height,
        )
    }
}

impl<M: Clone + PartialEq + 'static> From<ComboBox<M>> for Component<M> {
    fn from(cb: ComboBox<M>) -> Self {
        Component::new(cb)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for ComboBox<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        let closed_style = Style::default()
            .back_color(WHITE)
            .border(BLACK)
            .font_color(BLACK);
        let box_shape: Box<dyn ShapeGraph> = Box::new(self.bounds);
        paint_brush.draw_shape(&box_shape, closed_style);
        let label = self
            .items
            .get(self.selected)
            .map(|s| s.as_str())
            .unwrap_or("");
        paint_brush.draw_text(font_map, &self.bounds, label, closed_style.get_font_color());
        if self.open {
            let drop = self.dropdown_rect();
            let bg: Box<dyn ShapeGraph> = Box::new(drop);
            paint_brush.draw_shape(&bg, Style::default().back_color(WHITE).border(BLACK));
            for (i, item) in self.items.iter().enumerate() {
                let row = self.item_rect(i);
                let style = if i == self.selected {
                    Style::default().back_color(LIGHT_BLUE).font_color(BLACK)
                } else {
                    Style::default().back_color(WHITE).font_color(BLACK)
                };
                let row_shape: Box<dyn ShapeGraph> = Box::new(row);
                paint_brush.draw_shape(&row_shape, style);
                paint_brush.draw_text(font_map, &row, item.as_str(), style.get_font_color());
            }
        }
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let cursor = event_context.get_cursor_pos();
        let on_box = self.bounds.contain_coord(cursor);
        let on_drop = self.open && self.dropdown_rect().contain_coord(cursor);
        if let EventType::Mouse(Mouse::Left) = g_event.event {
            if g_event.state == State::Pressed {
                if on_box {
                    let next = !self.open;
                    event_context.send_message((self.on_toggle)(next));
                    return true;
                }
                if on_drop {
                    let rel_y = cursor.y - self.dropdown_rect().position.y;
                    let index = (rel_y as usize / self.item_height as usize).min(self.items.len());
                    if index < self.items.len() {
                        event_context.send_message((self.on_select)(index));
                        return true;
                    }
                }
                if self.open && !on_box && !on_drop {
                    event_context.send_message((self.on_toggle)(false));
                    return true;
                }
            }
        }
        false
    }
}
