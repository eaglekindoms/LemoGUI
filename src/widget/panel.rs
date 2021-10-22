use crate::graphic::render_middle::RenderUtil;
use crate::widget::{Component, ComponentModel};

pub struct Panel<M> where M: PartialEq, M: std::marker::Copy {
    pub widgets: Vec<Component<M>>,
}

impl<M: Copy + PartialEq> Panel<M> {
    pub fn new() -> Panel<M> {
        Panel {
            widgets: Vec::with_capacity(4)
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


impl<'a, M: Copy + PartialEq> ComponentModel<M> for Panel<M> {
    fn draw(&self, render_utils: &mut RenderUtil) {
        for widget in &self.widgets {
            widget.widget.draw(render_utils);
        }
    }
}