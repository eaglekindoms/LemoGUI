use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

/// 列表视图控件结构体
pub struct ListView<M: Clone> {
    /// 列表项
    pub items: Vec<String>,
    /// 列表区域
    pub bounds: Rectangle,
    /// 每项高度
    pub item_height: u32,
    /// 滚动偏移量（第几项开始显示）
    pub scroll_offset: usize,
    /// 当前选中项索引
    pub selected: Option<usize>,
    /// 选中回调
    pub on_select: Option<Box<dyn Fn(usize) -> M>>,
}

impl<M: Clone + PartialEq> ListView<M> {
    pub fn new<S: Into<String>, I: IntoIterator<Item = S>>(bounds: Rectangle, items: I) -> Self {
        Self {
            items: items.into_iter().map(|s| s.into()).collect(),
            bounds,
            item_height: 30,
            scroll_offset: 0,
            selected: None,
            on_select: None,
        }
    }

    pub fn on_select<F: 'static + Fn(usize) -> M>(mut self, f: F) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }

    pub fn item_height(mut self, h: u32) -> Self {
        self.item_height = h;
        self
    }

    fn visible_range(&self) -> std::ops::Range<usize> {
        let visible_count = (self.bounds.height / self.item_height) as usize;
        let start = self.scroll_offset;
        let end = (start + visible_count).min(self.items.len());
        start..end
    }
}

impl<M: Clone + PartialEq + 'static> From<ListView<M>> for Component<M> {
    fn from(lv: ListView<M>) -> Self {
        Component::new(lv)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for ListView<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        let bg: Box<dyn ShapeGraph> = Box::new(self.bounds);
        paint_brush.draw_shape(&bg, Style::default().back_color(WHITE).border(BLACK));
        let range = self.visible_range();
        for (i, item) in self.items[range.clone()].iter().enumerate() {
            let abs_index = range.start + i;
            let y = self.bounds.position.y + (i as u32 * self.item_height) as f32;
            let row_rect = Rectangle::new(
                self.bounds.position.x,
                y,
                self.bounds.width,
                self.item_height,
            );
            let row_style = if Some(abs_index) == self.selected {
                Style::default().back_color(LIGHT_BLUE).font_color(BLACK)
            } else {
                Style::default().back_color(WHITE).font_color(BLACK)
            };
            let row_shape: Box<dyn ShapeGraph> = Box::new(row_rect);
            paint_brush.draw_shape(&row_shape, row_style);
            paint_brush.draw_text(font_map, &row_rect, item.as_str(), row_style.get_font_color());
        }
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let cursor = event_context.get_cursor_pos();
        let hover = self.bounds.contain_coord(cursor);
        if let EventType::Mouse(Mouse::Left) = g_event.event {
            if g_event.state == State::Pressed && hover {
                let rel_y = cursor.y - self.bounds.position.y;
                let clicked_index =
                    self.scroll_offset + (rel_y as usize / self.item_height as usize);
                if clicked_index < self.items.len() {
                    self.selected = Some(clicked_index);
                    if let Some(on_select) = &self.on_select {
                        event_context.send_message(on_select(clicked_index));
                    }
                    return true;
                }
            }
        }
        hover
    }
}
