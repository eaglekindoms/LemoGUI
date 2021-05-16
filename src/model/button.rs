use std::fmt::Debug;

use wgpu::{Device, RenderPipeline};
use winit::event::WindowEvent;

use crate::device::display_window::DisplayWindow;
use crate::device::listener::Listener;
use crate::graphic::render_type::render_function::RenderGraph;
use crate::graphic::shape::point2d::{Point, RGBA};
use crate::graphic::shape::rectangle::Rectangle;
use crate::model::component::Component;
use crate::model::component::ComponentModel;

/// 按钮属性：矩形，背景颜色，聚焦颜色，文字颜色，文本内容
#[derive(Debug)]
pub struct Button<'a, L: Listener + ?Sized> {
    pub context: Component<'a, L>,
    pub index: Option<usize>,
}

impl<'a> Button<'a, Listener> {
    pub fn new(rect: Rectangle, text: &'a str) -> Self {
        let font_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let background_color = RGBA([0.8, 0.8, 0.8, 1.0]);
        let border_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let hover_color = RGBA([0.5, 0.0, 0.5, 0.5]);
        log::info!("create the Button obj");
        let context = Component::default(rect, font_color, background_color, border_color, hover_color, text);

        Self {
            context,
            index: None,
        }
    }

    pub fn default(pos: Point, text: &'a str) -> Self {
        let font_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let background_color = RGBA([0.8, 0.8, 0.8, 1.0]);
        let border_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let hover_color = RGBA([0.5, 0.0, 0.5, 0.5]);
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32, 40);
        log::info!("create the Button obj");
        let context = Component::default(rect, font_color, background_color, border_color, hover_color, text);

        Self {
            context,
            index: None,
        }
    }
}

impl<'a> ComponentModel for Button<'a, Listener> {
    fn set_index(&mut self, index: usize) {
        self.index = Option::from(index);
    }

    fn get_index(&self) -> Option<usize> {
        self.index
    }

    fn to_graph(&self, display_window: &DisplayWindow) -> RenderGraph {
        self.context.to_graph(display_window)
    }
}

impl<'a> Listener for Button<'a, Listener> {
    fn key_listener(&mut self, event: &WindowEvent) {
        log::info!("{:?}", event);
    }
}