use std::fmt::*;

use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

pub type KeyCode = VirtualKeyCode;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum State {
    Pressed,
    Released,
    None,
}

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

/// 组件状态结构体，记录绑定的事件、及与事件联动的消息
#[derive(Debug)]
pub struct GEvent {
    pub event: EventType,
    pub state: State,
}


impl From<winit::event::MouseButton> for Mouse {
    fn from(winit_mouse: MouseButton) -> Self {
        match winit_mouse {
            MouseButton::Left => { Mouse::Left }
            MouseButton::Right => { Mouse::Right }
            MouseButton::Middle => { Mouse::Middle }
            MouseButton::Other(_) => { Mouse::Other }
        }
    }
}

impl From<winit::event::ElementState> for State {
    fn from(winit_state: ElementState) -> Self {
        match winit_state {
            ElementState::Pressed => { State::Pressed }
            ElementState::Released => { State::Released }
        }
    }
}

impl From<&winit::event::WindowEvent<'_>> for GEvent {
    fn from(winit_event: &WindowEvent) -> Self {
        match winit_event {
            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => {
                GEvent {
                    event: EventType::Mouse((*button).into()),
                    state: (*state).into(),
                }
            }
            WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state,
                    virtual_keycode,
                    ..
                },
                ..
            } => {
                GEvent {
                    event: EventType::KeyBoard(*virtual_keycode),
                    state: (*state).into(),
                }
            }
            WindowEvent::ReceivedCharacter(c) => {
                GEvent {
                    event: EventType::ReceivedCharacter(*c),
                    state: State::None,
                }
            }
            _ => {
                GEvent {
                    event: EventType::Other,
                    state: State::None,
                }
            }
        }
    }
}