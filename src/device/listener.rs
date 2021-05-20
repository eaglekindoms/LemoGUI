use winit::event::{KeyboardInput, WindowEvent};

pub trait Listener {
    fn key_listener(&mut self, event: &WindowEvent);
    fn set_key_listener(&self, key: KeyboardInput) {}
}