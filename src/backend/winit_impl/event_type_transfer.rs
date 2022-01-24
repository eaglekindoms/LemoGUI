use winit::dpi::PhysicalSize;
use winit::event::*;

use crate::graphic::base::Point;
use crate::widget::*;

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
            MouseButton::Other(_) => Mouse::Other,
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
impl From<winit::event::WindowEvent<'_>> for GEvent {
    fn from(winit_event: WindowEvent) -> Self {
        match winit_event {
            WindowEvent::MouseInput { state, button, .. } => GEvent {
                event: EventType::Mouse((button).into()),
                state: state.into(),
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode,
                        ..
                    },
                ..
            } => {
                let mut keycode = None;
                if let Some(key) = virtual_keycode {
                    keycode = Some(translate_key(key))
                };
                GEvent {
                    event: EventType::KeyBoard(keycode),
                    state: state.into(),
                }
            }
            WindowEvent::ReceivedCharacter(c) => GEvent {
                event: EventType::ReceivedCharacter(c),
                state: State::None,
            },
            _ => GEvent {
                event: EventType::Other,
                state: State::None,
            },
        }
    }
}

/// 键盘按键类型转换
pub(crate) fn translate_key(key: VirtualKeyCode) -> KeyCode {
    match key {
        VirtualKeyCode::Key1 => KeyCode::Key1,
        VirtualKeyCode::Key2 => KeyCode::Key2,
        VirtualKeyCode::Key3 => KeyCode::Key3,
        VirtualKeyCode::Key4 => KeyCode::Key4,
        VirtualKeyCode::Key5 => KeyCode::Key5,
        VirtualKeyCode::Key6 => KeyCode::Key6,
        VirtualKeyCode::Key7 => KeyCode::Key7,
        VirtualKeyCode::Key8 => KeyCode::Key8,
        VirtualKeyCode::Key9 => KeyCode::Key9,
        VirtualKeyCode::Key0 => KeyCode::Key0,
        VirtualKeyCode::A => KeyCode::A,
        VirtualKeyCode::B => KeyCode::B,
        VirtualKeyCode::C => KeyCode::C,
        VirtualKeyCode::D => KeyCode::D,
        VirtualKeyCode::E => KeyCode::E,
        VirtualKeyCode::F => KeyCode::F,
        VirtualKeyCode::G => KeyCode::G,
        VirtualKeyCode::H => KeyCode::H,
        VirtualKeyCode::I => KeyCode::I,
        VirtualKeyCode::J => KeyCode::J,
        VirtualKeyCode::K => KeyCode::K,
        VirtualKeyCode::L => KeyCode::L,
        VirtualKeyCode::M => KeyCode::M,
        VirtualKeyCode::N => KeyCode::N,
        VirtualKeyCode::O => KeyCode::O,
        VirtualKeyCode::P => KeyCode::P,
        VirtualKeyCode::Q => KeyCode::Q,
        VirtualKeyCode::R => KeyCode::R,
        VirtualKeyCode::S => KeyCode::S,
        VirtualKeyCode::T => KeyCode::T,
        VirtualKeyCode::U => KeyCode::U,
        VirtualKeyCode::V => KeyCode::V,
        VirtualKeyCode::W => KeyCode::W,
        VirtualKeyCode::X => KeyCode::X,
        VirtualKeyCode::Y => KeyCode::Y,
        VirtualKeyCode::Z => KeyCode::Z,
        VirtualKeyCode::Escape => KeyCode::Escape,
        VirtualKeyCode::F1 => KeyCode::F1,
        VirtualKeyCode::F2 => KeyCode::F2,
        VirtualKeyCode::F3 => KeyCode::F3,
        VirtualKeyCode::F4 => KeyCode::F4,
        VirtualKeyCode::F5 => KeyCode::F5,
        VirtualKeyCode::F6 => KeyCode::F6,
        VirtualKeyCode::F7 => KeyCode::F7,
        VirtualKeyCode::F8 => KeyCode::F8,
        VirtualKeyCode::F9 => KeyCode::F9,
        VirtualKeyCode::F10 => KeyCode::F10,
        VirtualKeyCode::F11 => KeyCode::F11,
        VirtualKeyCode::F12 => KeyCode::F12,
        VirtualKeyCode::F13 => KeyCode::F13,
        VirtualKeyCode::F14 => KeyCode::F14,
        VirtualKeyCode::F15 => KeyCode::F15,
        VirtualKeyCode::F16 => KeyCode::F16,
        VirtualKeyCode::F17 => KeyCode::F17,
        VirtualKeyCode::F18 => KeyCode::F18,
        VirtualKeyCode::F19 => KeyCode::F19,
        VirtualKeyCode::F20 => KeyCode::F20,
        VirtualKeyCode::F21 => KeyCode::F21,
        VirtualKeyCode::F22 => KeyCode::F22,
        VirtualKeyCode::F23 => KeyCode::F23,
        VirtualKeyCode::F24 => KeyCode::F24,
        VirtualKeyCode::Snapshot => KeyCode::Snapshot,
        VirtualKeyCode::Scroll => KeyCode::Scroll,
        VirtualKeyCode::Pause => KeyCode::Pause,
        VirtualKeyCode::Insert => KeyCode::Insert,
        VirtualKeyCode::Home => KeyCode::Home,
        VirtualKeyCode::Delete => KeyCode::Delete,
        VirtualKeyCode::End => KeyCode::End,
        VirtualKeyCode::PageDown => KeyCode::PageDown,
        VirtualKeyCode::PageUp => KeyCode::PageUp,
        VirtualKeyCode::Left => KeyCode::Left,
        VirtualKeyCode::Up => KeyCode::Up,
        VirtualKeyCode::Right => KeyCode::Right,
        VirtualKeyCode::Down => KeyCode::Down,
        VirtualKeyCode::Back => KeyCode::Backspace,
        VirtualKeyCode::Return => KeyCode::Return,
        VirtualKeyCode::Space => KeyCode::Space,
        VirtualKeyCode::Compose => KeyCode::Compose,
        VirtualKeyCode::Caret => KeyCode::Caret,
        VirtualKeyCode::Numlock => KeyCode::Numlock,
        VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
        VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
        VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
        VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
        VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
        VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
        VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
        VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
        VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
        VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
        VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
        VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
        VirtualKeyCode::NumpadAdd => KeyCode::NumpadAdd,
        VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
        VirtualKeyCode::Apps => KeyCode::Apps,
        VirtualKeyCode::At => KeyCode::At,
        VirtualKeyCode::Ax => KeyCode::Ax,
        VirtualKeyCode::Backslash => KeyCode::Backslash,
        VirtualKeyCode::Calculator => KeyCode::Calculator,
        VirtualKeyCode::Capital => KeyCode::Capital,
        VirtualKeyCode::Colon => KeyCode::Colon,
        VirtualKeyCode::Comma => KeyCode::Comma,
        VirtualKeyCode::Convert => KeyCode::Convert,
        VirtualKeyCode::NumpadDecimal => KeyCode::NumpadDecimal,
        VirtualKeyCode::NumpadDivide => KeyCode::NumpadDivide,
        VirtualKeyCode::Equals => KeyCode::Equals,
        VirtualKeyCode::Grave => KeyCode::Grave,
        VirtualKeyCode::Kana => KeyCode::Kana,
        VirtualKeyCode::Kanji => KeyCode::Kanji,
        VirtualKeyCode::LAlt => KeyCode::LAlt,
        VirtualKeyCode::LBracket => KeyCode::LBracket,
        VirtualKeyCode::LControl => KeyCode::LControl,
        VirtualKeyCode::LShift => KeyCode::LShift,
        VirtualKeyCode::LWin => KeyCode::LWin,
        VirtualKeyCode::Mail => KeyCode::Mail,
        VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
        VirtualKeyCode::MediaStop => KeyCode::MediaStop,
        VirtualKeyCode::Minus => KeyCode::Minus,
        VirtualKeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
        VirtualKeyCode::Mute => KeyCode::Mute,
        VirtualKeyCode::MyComputer => KeyCode::MyComputer,
        VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
        VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
        VirtualKeyCode::NextTrack => KeyCode::NextTrack,
        VirtualKeyCode::NoConvert => KeyCode::NoConvert,
        VirtualKeyCode::NumpadComma => KeyCode::NumpadComma,
        VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
        VirtualKeyCode::NumpadEquals => KeyCode::NumpadEquals,
        VirtualKeyCode::OEM102 => KeyCode::OEM102,
        VirtualKeyCode::Period => KeyCode::Period,
        VirtualKeyCode::PlayPause => KeyCode::PlayPause,
        VirtualKeyCode::Power => KeyCode::Power,
        VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
        VirtualKeyCode::RAlt => KeyCode::RAlt,
        VirtualKeyCode::RBracket => KeyCode::RBracket,
        VirtualKeyCode::RControl => KeyCode::RControl,
        VirtualKeyCode::RShift => KeyCode::RShift,
        VirtualKeyCode::RWin => KeyCode::RWin,
        VirtualKeyCode::Semicolon => KeyCode::Semicolon,
        VirtualKeyCode::Slash => KeyCode::Slash,
        VirtualKeyCode::Sleep => KeyCode::Sleep,
        VirtualKeyCode::Stop => KeyCode::Stop,
        VirtualKeyCode::NumpadSubtract => KeyCode::NumpadSubtract,
        VirtualKeyCode::Sysrq => KeyCode::Sysrq,
        VirtualKeyCode::Tab => KeyCode::Tab,
        VirtualKeyCode::Underline => KeyCode::Underline,
        VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
        VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
        VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
        VirtualKeyCode::Wake => KeyCode::Wake,
        VirtualKeyCode::WebBack => KeyCode::WebBack,
        VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
        VirtualKeyCode::WebForward => KeyCode::WebForward,
        VirtualKeyCode::WebHome => KeyCode::WebHome,
        VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
        VirtualKeyCode::WebSearch => KeyCode::WebSearch,
        VirtualKeyCode::WebStop => KeyCode::WebStop,
        VirtualKeyCode::Yen => KeyCode::Yen,
        VirtualKeyCode::Copy => KeyCode::Copy,
        VirtualKeyCode::Paste => KeyCode::Paste,
        VirtualKeyCode::Cut => KeyCode::Cut,
        VirtualKeyCode::Asterisk => KeyCode::Asterisk,
        VirtualKeyCode::Plus => KeyCode::Plus,
    }
}
