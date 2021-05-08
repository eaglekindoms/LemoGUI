use winit::event::WindowEvent;

pub trait Listener {
    fn key_listener(&mut self, event: &WindowEvent);
}