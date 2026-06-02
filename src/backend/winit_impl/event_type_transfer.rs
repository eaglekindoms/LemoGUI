use winit::dpi::PhysicalSize;
use winit::event::*;
use winit::keyboard::{KeyCode as WKeyCode, PhysicalKey};

use crate::event::*;
use crate::graphic::base::Point;

impl From<Point<f32>> for winit::dpi::Position {
    #[inline]
    fn from(position: Point<f32>) -> winit::dpi::Position {
        winit::dpi::Position::Physical(winit::dpi::PhysicalPosition {
            x: position.x as i32,
            y: position.y as i32,
        })
    }
}

impl From<winit::dpi::PhysicalPosition<f64>> for Point<f32> {
    #[inline]
    fn from(position: winit::dpi::PhysicalPosition<f64>) -> Point<f32> {
        Point::new(position.x as f32, position.y as f32)
    }
}

impl From<winit::dpi::PhysicalSize<u32>> for Point<u32> {
    fn from(position: PhysicalSize<u32>) -> Self {
        Point::new(position.width, position.height)
    }
}

/// winit鼠标事件转换
impl From<winit::event::MouseButton> for Mouse {
    fn from(winit_mouse: MouseButton) -> Self {
        match winit_mouse {
            MouseButton::Left => Mouse::Left,
            MouseButton::Right => Mouse::Right,
            MouseButton::Middle => Mouse::Middle,
            _ => Mouse::Other,
        }
    }
}

/// winit事件状态转换
impl From<winit::event::ElementState> for State {
    fn from(winit_state: ElementState) -> Self {
        match winit_state {
            ElementState::Pressed => State::Pressed,
            ElementState::Released => State::Released,
        }
    }
}

/// winit事件转换
impl From<winit::event::WindowEvent> for GEvent {
    fn from(winit_event: WindowEvent) -> Self {
        match winit_event {
            WindowEvent::MouseInput { state, button, .. } => GEvent {
                event: EventType::Mouse((button).into()),
                state: state.into(),
            },
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key,
                        text,
                        ..
                    },
                ..
            } => {
                let keycode = if let PhysicalKey::Code(code) = physical_key {
                    Some(translate_key(code))
                } else {
                    None
                };
                // Handle text input (replaces ReceivedCharacter)
                if let Some(txt) = text {
                    if let Some(c) = txt.chars().next() {
                        if !c.is_control() {
                            return GEvent {
                                event: EventType::ReceivedCharacter(c),
                                state: State::None,
                            };
                        }
                    }
                }
                GEvent {
                    event: EventType::KeyBoard(keycode),
                    state: state.into(),
                }
            }
            _ => GEvent {
                event: EventType::Other,
                state: State::None,
            },
        }
    }
}

/// 键盘按键类型转换
pub(crate) fn translate_key(key: WKeyCode) -> KeyCode {
    macro_rules! map_keys {
        ($($wk:ident => $gk:ident),* $(,)?) => {
            match key {
                $(WKeyCode::$wk => KeyCode::$gk,)*
                _ => KeyCode::Unlabeled,
            }
        };
    }
    map_keys! {
        Digit1 => Key1, Digit2 => Key2, Digit3 => Key3, Digit4 => Key4, Digit5 => Key5,
        Digit6 => Key6, Digit7 => Key7, Digit8 => Key8, Digit9 => Key9, Digit0 => Key0,
        KeyA => A, KeyB => B, KeyC => C, KeyD => D, KeyE => E, KeyF => F, KeyG => G,
        KeyH => H, KeyI => I, KeyJ => J, KeyK => K, KeyL => L, KeyM => M, KeyN => N,
        KeyO => O, KeyP => P, KeyQ => Q, KeyR => R, KeyS => S, KeyT => T, KeyU => U,
        KeyV => V, KeyW => W, KeyX => X, KeyY => Y, KeyZ => Z,
        Escape => Escape,
        F1 => F1, F2 => F2, F3 => F3, F4 => F4, F5 => F5, F6 => F6,
        F7 => F7, F8 => F8, F9 => F9, F10 => F10, F11 => F11, F12 => F12,
        F13 => F13, F14 => F14, F15 => F15, F16 => F16, F17 => F17, F18 => F18,
        F19 => F19, F20 => F20, F21 => F21, F22 => F22, F23 => F23, F24 => F24,
        PrintScreen => Snapshot, ScrollLock => Scroll, Pause => Pause,
        Insert => Insert, Home => Home, Delete => Delete, End => End,
        PageDown => PageDown, PageUp => PageUp,
        ArrowLeft => Left, ArrowUp => Up, ArrowRight => Right, ArrowDown => Down,
        Backspace => Backspace, Enter => Return, Space => Space, Tab => Tab,
        CapsLock => Capital, NumLock => Numlock,
        Numpad0 => Numpad0, Numpad1 => Numpad1, Numpad2 => Numpad2, Numpad3 => Numpad3,
        Numpad4 => Numpad4, Numpad5 => Numpad5, Numpad6 => Numpad6, Numpad7 => Numpad7,
        Numpad8 => Numpad8, Numpad9 => Numpad9,
        NumpadAdd => NumpadAdd, NumpadSubtract => NumpadSubtract,
        NumpadMultiply => NumpadMultiply, NumpadDivide => NumpadDivide,
        NumpadDecimal => NumpadDecimal, NumpadComma => NumpadComma,
        NumpadEnter => NumpadEnter, NumpadEqual => NumpadEquals,
        AltLeft => LAlt, AltRight => RAlt,
        ControlLeft => LControl, ControlRight => RControl,
        ShiftLeft => LShift, ShiftRight => RShift,
        SuperLeft => LWin, SuperRight => RWin,
        Minus => Minus, Equal => Equals, Backquote => Grave,
        BracketLeft => LBracket, BracketRight => RBracket,
        Backslash => Backslash, Semicolon => Semicolon,
        Quote => Apostrophe, Comma => Comma, Slash => Slash, Period => Period,
    }
}
