use winit::event::WindowEvent;

use crate::device::display_window::{DisplayWindow, WGContext};
use crate::device::listener::Listener;
use crate::graphic::render_type::render_function::RenderGraph;
use crate::model::component::ComponentModel;

pub trait Painter: Sized {
    fn new(wgcontext: WGContext) -> Self;
    fn add_comp<C>(&mut self, comp: &mut C)
        where C: ComponentModel + Listener
    ;
    // fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    fn input(&mut self, event: &WindowEvent) -> bool;
    fn update(&mut self);
    fn render(&mut self);
}


