use std::fmt::*;

use winit::event::VirtualKeyCode;

type CallBack = Box<dyn Fn()>;

/// 控件状态结构体
/// 作用：记录控件当前聚焦的事件
// #[derive(Debug, Default)]
pub struct Message {
    keyboard: Option<VirtualKeyCode>,
    // action: Option<CallBack>,
}

impl Message {
    pub fn key(key: VirtualKeyCode) -> Message {
        Message {
            keyboard: Some(key),
            // action: None,
        }
    }

    // pub fn set_key(mut self, key: VirtualKeyCode, callback: Option<CallBack>) -> Self {
    //     self.keyboard = Some((key, callback));
    //     self
    // }

    pub fn get_key(&self) -> Option<VirtualKeyCode> {
        self.keyboard
    }
    // pub fn get_key_callback(&self) -> &Option<CallBack> {
    //     if let Some((_, callback)) = &self.keyboard {
    //         return callback
    //     }
    //     return &None;
    // }
    // pub fn action(callback: CallBack) -> Message {
    //     Message {
    //         keyboard: None,
    //         action: Some(callback),
    //     }
    // }
    //
    // pub fn set_action(mut self, callback: CallBack) -> Self {
    //     self.action = Some(callback);
    //     self
    // }
    //
    // pub fn get_action_callback(&self) -> &Option<CallBack> {
    //     &self.action
    // }
}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("key", &self.get_key())
            // .field("action", &self.action.is_some())
            .finish()
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Message {}