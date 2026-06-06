use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

/// 滚动条方向
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// 滚动条控件结构体
pub struct Scrollbar<M: Clone> {
    /// 轨道区域
    pub track: Rectangle,
    /// 方向
    pub orientation: Orientation,
    /// 当前值 (0.0 ~ 1.0)
    pub value: f32,
    /// 是否正在拖拽
    pub dragging: bool,
    /// 值变化回调
    pub on_change: Box<dyn Fn(f32) -> M>,
}

impl<M: Clone + PartialEq> Scrollbar<M> {
    pub fn new<F>(track: Rectangle, orientation: Orientation, on_change: F) -> Self
    where
        F: 'static + Fn(f32) -> M,
    {
        Self {
            track,
            orientation,
            value: 0.0,
            dragging: false,
            on_change: Box::new(on_change),
        }
    }

    pub fn value(mut self, v: f32) -> Self {
        self.value = v.clamp(0.0, 1.0);
        self
    }

    fn thumb_size(&self) -> u32 {
        20
    }

    fn thumb_rect(&self) -> Rectangle {
        let t = self.thumb_size();
        match self.orientation {
            Orientation::Horizontal => {
                let usable = self.track.width.saturating_sub(t);
                let x = self.track.position.x + usable as f32 * self.value;
                Rectangle::new(x, self.track.position.y, t, self.track.height)
            }
            Orientation::Vertical => {
                let usable = self.track.height.saturating_sub(t);
                let y = self.track.position.y + usable as f32 * self.value;
                Rectangle::new(self.track.position.x, y, self.track.width, t)
            }
        }
    }

    fn update_from_cursor(&mut self, cursor: Point<f32>) {
        let t = self.thumb_size() as f32;
        self.value = match self.orientation {
            Orientation::Horizontal => {
                let usable = self.track.width as f32 - t;
                if usable <= 0.0 {
                    0.0
                } else {
                    ((cursor.x - self.track.position.x - t / 2.0) / usable).clamp(0.0, 1.0)
                }
            }
            Orientation::Vertical => {
                let usable = self.track.height as f32 - t;
                if usable <= 0.0 {
                    0.0
                } else {
                    ((cursor.y - self.track.position.y - t / 2.0) / usable).clamp(0.0, 1.0)
                }
            }
        };
    }
}

impl<M: Clone + PartialEq + 'static> From<Scrollbar<M>> for Component<M> {
    fn from(sb: Scrollbar<M>) -> Self {
        Component::new(sb)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for Scrollbar<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, _font_map: &mut GCharMap) {
        let track_shape: Box<dyn ShapeGraph> = Box::new(self.track);
        paint_brush.draw_shape(
            &track_shape,
            Style::default().back_color(LIGHT_WHITE).no_border(),
        );
        let thumb = self.thumb_rect();
        let thumb_shape: Box<dyn ShapeGraph> = Box::new(thumb);
        paint_brush.draw_shape(
            &thumb_shape,
            Style::default().back_color(LIGHT_BLUE).border(BLACK).round(),
        );
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let cursor = event_context.get_cursor_pos();
        let thumb = self.thumb_rect();
        match g_event.event {
            EventType::Mouse(Mouse::Left) => {
                if g_event.state == State::Pressed && thumb.contain_coord(cursor) {
                    self.dragging = true;
                } else if g_event.state == State::Released {
                    self.dragging = false;
                }
            }
            _ => {
                if self.dragging {
                    let old = self.value;
                    self.update_from_cursor(cursor);
                    if (self.value - old).abs() > f32::EPSILON {
                        event_context.send_message((self.on_change)(self.value));
                    }
                }
            }
        }
        thumb.contain_coord(cursor) || self.dragging
    }
}
