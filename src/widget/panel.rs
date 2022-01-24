use crate::graphic::base::GCharMap;
use crate::graphic::render_api::PaintBrush;
use crate::widget::{Component, ComponentModel, EventContext};

/// 容器面板结构体
#[derive(Debug)]
pub struct Panel<M>
where
    M: PartialEq,
    M: std::clone::Clone,
{
    pub widgets: Vec<Component<M>>,
}

impl<M: Clone + PartialEq> Panel<M> {
    pub fn new() -> Panel<M> {
        Panel {
            widgets: Vec::with_capacity(4),
        }
    }

    pub fn push<E>(mut self, child: E) -> Self
    where
        E: Into<Component<M>>,
    {
        self.widgets.push(child.into());
        self
    }
}

impl<'a, M: Clone + PartialEq> ComponentModel<M> for Panel<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        for widget in &self.widgets {
            widget.widget.draw(paint_brush, font_map);
        }
    }
    fn listener(&mut self, _event_context: &mut dyn EventContext<M>) -> bool {
        let mut is_listener = false;
        for comp in &mut self.widgets {
            if comp.widget.listener(_event_context) {
                is_listener = true;
            }
        }
        return is_listener;
    }
}
