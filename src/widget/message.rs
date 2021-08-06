use std::fmt::*;

use winit::event::VirtualKeyCode;

type CallBack = Box<dyn Fn()>;

/// 控件状态结构体
/// 作用：记录控件当前聚焦的事件
// #[derive(Debug, Default)]
pub struct Message {
    keyboard: Option<(VirtualKeyCode, Option<CallBack>)>,
    action: Option<CallBack>,
}

impl Message {
    pub fn key(key: VirtualKeyCode, callback: Option<CallBack>) -> Message {
        Message {
            keyboard: Some((key, callback)),
            action: None,
        }
    }

    pub fn set_key(mut self, key: VirtualKeyCode, callback: Option<CallBack>) -> Self {
        self.keyboard = Some((key, callback));
        self
    }

    pub fn get_key(&self) -> Option<VirtualKeyCode> {
        if let Some((key, _)) = self.keyboard {
            return Some(key)
        }
        None
    }
    pub fn get_key_callback(&self) -> &Option<CallBack> {
        if let Some((_, callback)) = &self.keyboard {
            return callback
        }
        return &None;
    }
    pub fn action(callback: Box<dyn Fn()>) -> Message {
        Message {
            keyboard: None,
            action: Some(callback),
        }
    }

    pub fn set_action(mut self, callback: Box<dyn Fn()>) -> Self {
        self.action = Some(callback);
        self
    }

    pub fn get_action_callback(&self) -> &Option<CallBack> {
        &self.action
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("key", &self.get_key())
            .field("action", &self.action.is_some())
            .finish()
    }
}
