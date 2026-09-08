use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

/// 一排色块，点击发送颜色
pub struct ColorPalette<M: Clone> {
    pub origin: Point<f32>,
    pub swatch_size: u32,
    pub gap: f32,
    pub colors: Vec<RGBA>,
    pub selected: Option<usize>,
    pub on_pick: Box<dyn Fn(RGBA) -> M>,
}

impl<M: Clone + PartialEq> ColorPalette<M> {
    pub fn new<F>(origin: Point<f32>, on_pick: F) -> Self
    where
        F: 'static + Fn(RGBA) -> M,
    {
        Self {
            origin,
            swatch_size: 22,
            gap: 4.0,
            colors: vec![BLACK, RED, ORANGE, GREEN, BLUE, PURPLE, GRAY],
            selected: Some(0),
            on_pick: Box::new(on_pick),
        }
    }

    pub fn selected_color(mut self, color: RGBA) -> Self {
        self.selected = self.colors.iter().position(|c| *c == color);
        self
    }

    fn swatch_rect(&self, index: usize) -> Rectangle {
        let x = self.origin.x + index as f32 * (self.swatch_size as f32 + self.gap);
        Rectangle::new(self.origin.x.max(x), self.origin.y, self.swatch_size, self.swatch_size)
    }
}

impl<M: Clone + PartialEq + 'static> From<ColorPalette<M>> for Component<M> {
    fn from(cp: ColorPalette<M>) -> Self {
        Component::new(cp)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for ColorPalette<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, _font_map: &mut GCharMap) {
        for (i, color) in self.colors.iter().enumerate() {
            let rect = self.swatch_rect(i);
            let border = if Some(i) == self.selected {
                RGBA(0.1, 0.1, 0.1, 1.0)
            } else {
                LIGHT_WHITE
            };
            let shape: Box<dyn ShapeGraph> = Box::new(rect);
            paint_brush.draw_shape(&shape, Style::default().back_color(*color).border(border));
        }
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let cursor = event_context.get_cursor_pos();
        if let EventType::Mouse(Mouse::Left) = g_event.event {
            if g_event.state == State::Pressed {
                for (i, color) in self.colors.iter().enumerate() {
                    if self.swatch_rect(i).contain_coord(cursor) {
                        self.selected = Some(i);
                        event_context.send_message((self.on_pick)(*color));
                        return true;
                    }
                }
            }
        }
        false
    }
}
