use std::fmt::*;

use winit::event::VirtualKeyCode;

type CallBack = Box<dyn Fn()>;

/// 事件类型枚举
#[derive(Debug, PartialOrd, PartialEq)]
pub enum EventType {
    mouse,
    KeyBoard(VirtualKeyCode),
}

/// 组件状态结构体，记录绑定的事件、及与事件联动的消息
#[derive(Debug)]
pub struct State<M> {
    pub event: EventType,
    pub message: Option<M>,
}

impl<M: PartialEq> State<M> {
    pub fn match_message(&self, des_m: &M) -> bool {
        if self.message.is_some() {
            self.message.as_ref().unwrap() == des_m
        } else {
            false
        }
    }
}