use winit::event::WindowEvent;

use crate::device::display_window::WGContext;
use crate::widget::component::ComponentModel;
use crate::widget::listener::Listener;

pub trait Painter: Sized {
    fn new(wgcontext: WGContext) -> Self;
    fn add_comp<C>(&mut self, comp: &mut C)
        where C: ComponentModel + Listener + 'static
    ;
    // fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    fn input(&mut self, event: &WindowEvent) -> bool;
    fn update(&mut self);
    fn render(&mut self);
}


