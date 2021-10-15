use std::fmt::Debug;

pub use button::*;
pub use component::*;
pub use drawing_board::*;
pub use frame::*;
pub use message::*;
pub use text_input::*;

use crate::device::DisplayWindow;
use crate::graphic::render_middle::RenderUtil;

mod button;
mod component;
mod frame;
mod drawing_board;
mod message;
mod text_input;

pub struct Component<M> {
    widget: Box<dyn ComponentModel<M>>,
}

impl<M: Copy + PartialEq> Component<M> {
    pub fn new(widget: impl ComponentModel<M> + 'static) -> Component<M> {
        Component {
            widget: Box::new(widget)
        }
    }
}


impl<M: Copy + PartialEq + 'static> From<button::Button<M>> for Component<M> {
    fn from(button: button::Button<M>) -> Self {
        Component::new(button)
    }
}


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

/// 实例 trait
/// 用于定义具体应用
pub trait Instance {
    type M: 'static + Copy + PartialEq + Debug;
    /// 新建实例
    fn new() -> Self;
    /// 组件布局
    fn layout(&self) -> Panel<Self::M>;
    /// 状态更新
    fn update(&mut self, broadcast: &Self::M);
    /// 窗体设置
    fn setting() -> winit::window::WindowBuilder;
    /// 运行实例
    fn run() where Self: 'static + Sized {
        let window = DisplayWindow::new(Self::setting());
        let frame = window.request_container::<Frame<Self::M>>();
        window.start(frame, Self::new())
    }
}


impl<'a, M: Copy + PartialEq> ComponentModel<M> for Panel<M> {
    fn draw(&self, render_utils: &mut RenderUtil) {
        for widget in &self.widgets {
            widget.widget.draw(render_utils);
        }
    }
}

