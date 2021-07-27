use winit::event::*;

#[derive(Debug)]
pub struct State {
    pub keyboard: Option<VirtualKeyCode>,
}

impl State {
    pub fn new(key: Option<VirtualKeyCode>) -> State {
        State {
            keyboard: key
        }
    }

    pub fn get_key(&self) -> Option<&VirtualKeyCode> {
        self.keyboard.as_ref()
    }
}

pub trait Listener {
    fn key_listener(&mut self, event: &WindowEvent) -> bool {
        false
    }
}