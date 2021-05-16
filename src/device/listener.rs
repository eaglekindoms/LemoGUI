use winit::event::{WindowEvent, KeyboardInput};

pub trait Listener {
    fn key_listener(&mut self, event: &WindowEvent);
    fn set_key_listener(&self,key:KeyboardInput){

    }
}