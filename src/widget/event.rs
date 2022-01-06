use std::fmt::*;

pub type KeyCode = winit::event::VirtualKeyCode;

/// 控件点击状态结构体
#[derive(Debug, PartialOrd, PartialEq)]
pub enum State {
    Pressed,
    Released,
    None,
}

/// 鼠标事件结构体
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Mouse {
    Left,
    Right,
    Middle,
    Other,
}

/// 事件类型枚举
#[derive(Debug, PartialOrd, PartialEq)]
pub enum EventType {
    Mouse(Mouse),
    KeyBoard(Option<KeyCode>),
    ReceivedCharacter(char),
    Other,
}

/// 事件描述结构体
#[derive(Debug)]
pub struct BindEvent<M> {
    pub message: Option<M>,
    pub mouse: Mouse,
    pub shortcuts: Option<Vec<KeyCode>>,
}

/// 组件状态结构体，记录绑定的事件、及与事件联动的消息
#[derive(Debug)]
pub struct GEvent {
    pub event: EventType,
    pub state: State,
}

impl<M> Default for BindEvent<M> {
    fn default() -> Self {
        BindEvent {
            message: None,
            mouse: Mouse::Left,
            shortcuts: None,
        }
    }
}