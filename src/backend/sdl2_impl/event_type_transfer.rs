use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::event::*;

/// sdl2鼠标按键转换
impl From<sdl2::mouse::MouseButton> for Mouse {
    fn from(mouse_btn: MouseButton) -> Self {
        match mouse_btn {
            MouseButton::Unknown => Mouse::Other,
            MouseButton::Left => Mouse::Left,
            MouseButton::Middle => Mouse::Middle,
            MouseButton::Right => Mouse::Right,
            MouseButton::X1 => Mouse::Other,
            MouseButton::X2 => Mouse::Other,
        }
    }
}

/// sdl2事件转换
impl From<sdl2::event::Event> for GEvent {
    fn from(sdl2_event: Event) -> Self {
        match sdl2_event {
            Event::TextInput { text, .. } => {
                if let Some(c) = text.chars().next() {
                    GEvent {
                        event: EventType::ReceivedCharacter(c),
                        state: State::None,
                    }
                } else {
                    GEvent {
                        event: EventType::Other,
                        state: State::None,
                    }
                }
            }
            Event::KeyDown { keycode, .. } => {
                let mut key_code = None;
                if let Some(key) = keycode {
                    key_code = Some(translate_key(key))
                };
                GEvent {
                    event: EventType::KeyBoard(key_code),
                    state: State::Pressed,
                }
            }
            Event::KeyUp { keycode, .. } => {
                let mut key_code = None;
                if let Some(key) = keycode {
                    key_code = Some(translate_key(key))
                };
                GEvent {
                    event: EventType::KeyBoard(key_code),
                    state: State::Released,
                }
            }
            Event::MouseButtonDown { mouse_btn, .. } => GEvent {
                event: EventType::Mouse(mouse_btn.into()),
                state: State::Pressed,
            },
            Event::MouseButtonUp { mouse_btn, .. } => GEvent {
                event: EventType::Mouse(mouse_btn.into()),
                state: State::Released,
            },
            _ => GEvent {
                event: EventType::Other,
                state: State::None,
            },
        }
    }
}

/// 键盘按键类型转换
pub(crate) fn translate_key(key: Keycode) -> KeyCode {
    macro_rules! map_keys {
        ($($sk:ident => $gk:ident),* $(,)?) => {
            match key {
                $(Keycode::$sk => KeyCode::$gk,)*
                _ => KeyCode::Unlabeled,
            }
        };
    }
    map_keys! {
        Num0 => Numpad0, Num1 => Numpad1, Num2 => Numpad2, Num3 => Numpad3, Num4 => Numpad4,
        Num5 => Numpad5, Num6 => Numpad6, Num7 => Numpad7, Num8 => Numpad8, Num9 => Numpad9,
        A => A, B => B, C => C, D => D, E => E, F => F, G => G, H => H, I => I,
        J => J, K => K, L => L, M => M, N => N, O => O, P => P, Q => Q, R => R,
        S => S, T => T, U => U, V => V, W => W, X => X, Y => Y, Z => Z,
        Escape => Escape,
        F1 => F1, F2 => F2, F3 => F3, F4 => F4, F5 => F5, F6 => F6,
        F7 => F7, F8 => F8, F9 => F9, F10 => F10, F11 => F11, F12 => F12,
        F13 => F13, F14 => F14, F15 => F15, F16 => F16, F17 => F17, F18 => F18,
        F19 => F19, F20 => F20, F21 => F21, F22 => F22, F23 => F23, F24 => F24,
        PrintScreen => Snapshot, ScrollLock => Scroll, Pause => Pause,
        Insert => Insert, Home => Home, Delete => Delete, End => End,
        PageDown => PageDown, PageUp => PageUp,
        Left => Left, Up => Up, Right => Right, Down => Down,
        Backspace => Backspace, Return => Return, Space => Space, Tab => Tab,
        NumLockClear => Numlock,
        Kp0 => Numpad0, Kp1 => Numpad1, Kp2 => Numpad2, Kp3 => Numpad3,
        Kp4 => Numpad4, Kp5 => Numpad5, Kp6 => Numpad6, Kp7 => Numpad7,
        Kp8 => Numpad8, Kp9 => Numpad9,
        KpPlus => NumpadAdd, KpMinus => NumpadSubtract,
        KpMultiply => NumpadMultiply, KpDivide => NumpadDivide,
        KpDecimal => NumpadDecimal, KpComma => NumpadComma,
        KpEnter => NumpadEnter, KpEquals => NumpadEquals,
        Copy => Copy, Paste => Paste, Cut => Cut,
        At => At, Caret => Caret, Equals => Equals, Plus => Plus,
        Power => Power, Asterisk => Asterisk, Semicolon => Semicolon,
        Backslash => Backslash, Colon => Colon, Comma => Comma,
        Calculator => Calculator, Underscore => Underline, Period => Period,
        LAlt => LAlt, RAlt => RAlt,
        LeftBracket => LBracket, RightBracket => RBracket,
        LShift => LShift, RShift => RShift,
        LCtrl => LControl, RCtrl => RControl,
        LGui => LGui, RGui => RGui,
        VolumeDown => VolumeDown, VolumeUp => VolumeUp,
        Mail => Mail, MediaSelect => MediaSelect,
        Minus => Minus, Mute => Mute, Slash => Slash,
        Sleep => Sleep, Stop => Stop,
        Exclaim => Exclaim, Quotedbl => Quotedbl, Hash => Hash,
        Dollar => Dollar, Percent => Percent, Ampersand => Ampersand,
        Quote => Quote, LeftParen => LeftParen, RightParen => RightParen,
        Less => Less, Greater => Greater, Question => Question,
        Backquote => Backquote, CapsLock => CapsLock, KpPeriod => Period,
        Application => Application, Execute => Execute, Help => Help,
        Menu => Menu, Select => Select, Again => Again, Undo => Undo,
        Find => Find, KpEqualsAS400 => KpEqualsAS400, AltErase => AltErase,
        Sysreq => Sysreq, Cancel => Cancel, Clear => Clear,
        Prior => Period, Return2 => Return2, Separator => Separator,
        Out => Out, Oper => Oper, ClearAgain => ClearAgain,
        CrSel => CrSel, ExSel => ExSel,
        Kp00 => Kp00, Kp000 => Kp000,
        ThousandsSeparator => ThousandsSeparator, DecimalSeparator => DecimalSeparator,
        CurrencyUnit => CurrencyUnit, CurrencySubUnit => CurrencySubUnit,
        KpLeftParen => KpLeftParen, KpRightParen => KpRightParen,
        KpLeftBrace => KpLeftBrace, KpRightBrace => KpRightBrace,
        KpTab => Tab, KpBackspace => Backspace,
        KpA => KpA, KpB => KpB, KpC => KpC, KpD => KpD, KpE => KpE, KpF => KpF,
        KpXor => KpXor, KpPower => KpPower,
        KpPercent => Percent, KpLess => Less, KpGreater => Greater,
        KpAmpersand => Ampersand, KpDblAmpersand => KpDblAmpersand,
        KpVerticalBar => KpVerticalBar, KpDblVerticalBar => KpDblVerticalBar,
        KpColon => Colon, KpHash => Hash, KpSpace => Space,
        KpAt => At, KpExclam => Exclaim,
        KpMemStore => KpMemStore, KpMemRecall => KpMemRecall, KpMemClear => KpMemClear,
        KpMemAdd => KpMemAdd, KpMemSubtract => KpMemSubtract,
        KpMemMultiply => KpMemMultiply, KpMemDivide => KpMemDivide,
        KpPlusMinus => KpPlusMinus, KpClear => Clear, KpClearEntry => KpClearEntry,
        KpBinary => KpBinary, KpOctal => KpOctal, KpHexadecimal => KpHexadecimal,
        Mode => Mode,
        AudioNext => AudioNext, AudioPrev => AudioPrev, AudioStop => AudioStop,
        AudioPlay => AudioPlay, AudioMute => AudioMute,
        Www => Www, Computer => Computer,
        AcSearch => AcSearch, AcHome => AcHome, AcBack => AcBack,
        AcForward => AcForward, AcStop => AcStop, AcRefresh => AcRefresh,
        AcBookmarks => AcBookmarks,
        BrightnessDown => BrightnessDown, BrightnessUp => BrightnessUp,
        DisplaySwitch => DisplaySwitch,
        KbdIllumToggle => KbdIllumToggle, KbdIllumDown => KbdIllumDown,
        KbdIllumUp => KbdIllumUp, Eject => Eject,
    }
}
