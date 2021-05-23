use winit::event::{KeyboardInput, WindowEvent};

use crate::widget::component::ComponentModel;

pub trait Listener {
    fn key_listener(&mut self, event: &WindowEvent);
    fn set_key_listener(&self, key: KeyboardInput) {}
}