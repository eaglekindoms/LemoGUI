use winit::event::WindowEvent;

use crate::device::display_window::DisplayWindow;
use crate::device::listener::Listener;
use crate::graphic::render_type::render_function::RenderGraph;
use crate::model::component::ComponentModel;

pub trait Painter: Sized {
    fn new(display_window: &DisplayWindow) -> Self;
    fn add_comp<C: ComponentModel>(&mut self, display_device: &DisplayWindow, comp: C);
    // fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    fn input(&mut self, event: &WindowEvent) -> bool;
    fn update(&mut self);
    fn render(
        &mut self, display_window: &DisplayWindow,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
    ) -> Result<(), wgpu::SwapChainError>;
}


