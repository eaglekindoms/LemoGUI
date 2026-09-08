use std::cell::Cell;
use std::rc::Rc;

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

/// 编辑区与滑块共享的滚动状态（拖拽只写 Cell，不发消息）
#[derive(Debug)]
pub struct ScrollState {
    pub value: Cell<f32>,
    pub dragging: Cell<bool>,
    pub content_h: Cell<f32>,
    pub view_h: Cell<f32>,
}

impl ScrollState {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            value: Cell::new(0.0),
            dragging: Cell::new(false),
            content_h: Cell::new(0.0),
            view_h: Cell::new(0.0),
        })
    }

    pub fn ratio(&self) -> f32 {
        let c = self.content_h.get();
        let v = self.view_h.get();
        if c <= v || v <= 0.0 {
            1.0
        } else {
            (v / c).clamp(0.08, 1.0)
        }
    }

    pub fn can_scroll(&self) -> bool {
        self.content_h.get() > self.view_h.get() + 0.5
    }
}

/// 滚动条控件结构体
pub struct Scrollbar<M: Clone> {
    /// 轨道区域
    pub track: Rectangle,
    /// 方向
    pub orientation: Orientation,
    pub state: Rc<ScrollState>,
    /// 松手时写回 Instance
    pub on_release: Box<dyn Fn(f32) -> M>,
}

impl<M: Clone + PartialEq> Scrollbar<M> {
    pub fn new<F>(
        track: Rectangle,
        orientation: Orientation,
        state: Rc<ScrollState>,
        on_release: F,
    ) -> Self
    where
        F: 'static + Fn(f32) -> M,
    {
        Self {
            track,
            orientation,
            state,
            on_release: Box::new(on_release),
        }
    }

    fn thumb_len(&self) -> f32 {
        let track_len = match self.orientation {
            Orientation::Horizontal => self.track.width as f32,
            Orientation::Vertical => self.track.height as f32,
        };
        (track_len * self.state.ratio()).clamp(16.0, track_len.max(1.0))
    }

    fn thumb_rect(&self) -> Rectangle {
        let t = self.thumb_len();
        let value = self.state.value.get().clamp(0.0, 1.0);
        match self.orientation {
            Orientation::Horizontal => {
                let usable = (self.track.width as f32 - t).max(0.0);
                let x = self.track.position.x + usable * value;
                Rectangle::new(x, self.track.position.y, t as u32, self.track.height)
            }
            Orientation::Vertical => {
                let usable = (self.track.height as f32 - t).max(0.0);
                let y = self.track.position.y + usable * value;
                Rectangle::new(self.track.position.x, y, self.track.width, t as u32)
            }
        }
    }

    fn update_from_cursor(&self, cursor: Point<f32>) {
        let t = self.thumb_len();
        let value = match self.orientation {
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
        self.state.value.set(value);
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
        let color = if self.state.can_scroll() {
            LIGHT_BLUE
        } else {
            GRAY
        };
        let thumb_shape: Box<dyn ShapeGraph> = Box::new(thumb);
        paint_brush.draw_shape(
            &thumb_shape,
            Style::default().back_color(color).border(BLACK).round(),
        );
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let cursor = event_context.get_cursor_pos();
        let on_track = self.track.contain_coord(cursor);
        let dragging = self.state.dragging.get();
        let mut dirty = false;
        match g_event.event {
            EventType::Mouse(Mouse::Left) => {
                if g_event.state == State::Pressed && on_track && self.state.can_scroll() {
                    self.state.dragging.set(true);
                    self.update_from_cursor(cursor);
                    dirty = true;
                } else if g_event.state == State::Released && dragging {
                    self.state.dragging.set(false);
                    event_context.send_message((self.on_release)(self.state.value.get()));
                    dirty = true;
                }
            }
            _ => {
                if dragging && self.state.can_scroll() {
                    self.update_from_cursor(cursor);
                    dirty = true;
                }
            }
        }
        dirty || self.state.dragging.get()
    }
}
