use std::rc::Rc;

use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::widget::*;

/// 视口 + 子控件（滚动值由 Instance / 子控件自己持有）
pub struct ScrollPanel<M: Clone> {
    pub viewport: Rectangle,
    pub child: Component<M>,
}

impl<M: Clone + PartialEq> ScrollPanel<M> {
    pub fn new<C>(viewport: Rectangle, child: C) -> Self
    where
        C: Into<Component<M>>,
    {
        Self {
            viewport,
            child: child.into(),
        }
    }
}

impl<M: Clone + PartialEq + 'static> From<ScrollPanel<M>> for Component<M> {
    fn from(sp: ScrollPanel<M>) -> Self {
        Component::new(sp)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for ScrollPanel<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        self.child.widget.draw(paint_brush, font_map);
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        self.child.widget.listener(event_context)
    }

    fn ime_caret(&self) -> Option<(Point<f32>, f32)> {
        self.child.widget.ime_caret()
    }
}

/// 视口 + 子控件 + 纵向滚动条
pub struct ScrollViewer<M: Clone> {
    pub panel: ScrollPanel<M>,
    pub bar: Scrollbar<M>,
}

impl<M: Clone + PartialEq> ScrollViewer<M> {
    pub fn new<C, F>(
        viewport: Rectangle,
        child: C,
        state: Rc<ScrollState>,
        on_release: F,
    ) -> Self
    where
        C: Into<Component<M>>,
        F: 'static + Fn(f32) -> M,
    {
        let bar_w = 16u32;
        let track = Rectangle::new(
            viewport.position.x + viewport.width.saturating_sub(bar_w) as f32,
            viewport.position.y,
            bar_w,
            viewport.height,
        );
        Self {
            panel: ScrollPanel::new(viewport, child),
            bar: Scrollbar::new(track, Orientation::Vertical, state, on_release),
        }
    }
}

impl<M: Clone + PartialEq + 'static> From<ScrollViewer<M>> for Component<M> {
    fn from(sv: ScrollViewer<M>) -> Self {
        Component::new(sv)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for ScrollViewer<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        self.panel.draw(paint_brush, font_map);
        self.bar.draw(paint_brush, font_map);
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        if self.bar.listener(event_context) {
            return true;
        }
        self.panel.listener(event_context)
    }

    fn ime_caret(&self) -> Option<(Point<f32>, f32)> {
        self.panel.ime_caret()
    }
}
