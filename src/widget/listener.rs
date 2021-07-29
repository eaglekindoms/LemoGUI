use winit::event::*;

/// 控件状态结构体
/// 作用：记录控件当前聚焦的事件
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

/// 事件监听器
/// 作用：监听用户交互事件
pub trait Listener {
    /// 键盘事件监听器
    fn key_listener(&mut self, event: &WindowEvent) -> bool {
        false
    }

    /// 鼠标事件监听器
    fn mouse_listener(&mut self, event: &WindowEvent) -> bool { false }
}