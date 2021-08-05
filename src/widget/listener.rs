use winit::event::*;

use crate::graphic::base::shape::Point;

/// 控件状态结构体
/// 作用：记录控件当前聚焦的事件
#[derive(Debug, Default)]
pub struct Message {
    pub keyboard: Option<VirtualKeyCode>,
    pub mouse: Option<Point<f64>>,
}

impl Message {
    pub fn key(key: VirtualKeyCode) -> Message {
        Message {
            keyboard: Some(key),
            mouse: None,
        }
    }

    pub fn set_key(mut self, key: VirtualKeyCode) -> Self {
        self.keyboard = Some(key);
        self
    }

    pub fn get_key(&self) -> Option<&VirtualKeyCode> {
        self.keyboard.as_ref()
    }

    pub fn mouse(mouse: Point<f64>) -> Message {
        Message {
            keyboard: None,
            mouse: Some(mouse),
        }
    }

    pub fn set_mouse(mut self, mouse: Point<f64>) -> Self {
        self.mouse = Some(mouse);
        self
    }

    pub fn get_mouse(&self) -> Option<&Point<f64>> {
        self.mouse.as_ref()
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