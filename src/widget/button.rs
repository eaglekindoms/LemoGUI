use std::fmt::Debug;

use winit::event::WindowEvent;

use crate::device::display_window::WGContext;
use crate::graphic::base::point2d::Point;
use crate::graphic::base::rectangle::Rectangle;
use crate::graphic::render_middle::render_function::RenderGraph;
use crate::graphic::style::*;
use crate::widget::component::Component;
use crate::widget::component::ComponentModel;
use crate::widget::listener::Listener;

/// 按钮属性：矩形，背景颜色，聚焦颜色，文字颜色，文本内容
#[derive(Debug)]
pub struct Button<L: Listener + ?Sized> {
    pub context: Component<L>,
    pub index: Option<usize>,
}

impl<'a> Button<dyn Listener> {
    pub fn new_with_style(rect: Rectangle, style: Style, text: &'a str) -> Self {
        log::info!("create the Button obj use new");
        Self {
            context: Component::new(rect, style, text, None),
            index: None,
        }
    }

    pub fn new(pos: Point, text: &'a str) -> Self {
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32, 40);
        log::info!("create the Button obj use default");
        Self {
            context: Component::new(rect, Style::default(), text, None),
            index: None,
        }
    }
}

impl<'a> ComponentModel for Button<dyn Listener> {
    fn set_index(&mut self, index: usize) {
        self.index = Option::from(index);
    }

    fn get_index(&self) -> Option<usize> {
        self.index
    }

    fn to_graph(&mut self, wgcontext: &WGContext) -> &RenderGraph {
        self.context.to_graph(wgcontext)
    }

    fn get_style(&self) -> &Style {
        self.context.get_style()
    }
}

impl<'a> Listener for Button<dyn Listener> {
    fn key_listener(&mut self, event: &WindowEvent) {
        log::info!("{:?}", event);
    }
}