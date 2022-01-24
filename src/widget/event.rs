use std::fmt::*;

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

/// Describes the appearance of the mouse cursor.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cursor {
    /// The platform-dependent default cursor.
    Default,
    /// Indicates text that may be selected or edited.
    Text,
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

/// 键盘按键枚举
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum KeyCode {
    /// 1 !
    Key1,
    /// 2 @
    Key2,
    /// 3 #
    Key3,
    /// 4 $
    Key4,
    /// 5 %
    Key5,
    /// 6 ^
    Key6,
    /// 7 &
    Key7,
    /// 8 *
    Key8,
    /// 9 (
    Key9,
    /// 0 )
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    /// printScreen
    Snapshot,
    /// scroll lock
    Scroll,
    Pause,

    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    Backspace,
    Return,
    Space,

    LAlt,
    RAlt,
    LControl,
    RControl,
    LShift,
    RShift,
    LWin,
    RWin,

    /// special char
    /// - _
    Minus,
    /// = +
    Equals,
    /// ` ~
    Grave,
    /// [ {
    LBracket,
    /// ] }
    RBracket,
    /// \\  |
    Backslash,
    /// ; :
    Semicolon,
    /// :
    Colon,
    /// \' \"
    Apostrophe,
    /// , <
    Comma,
    /// /  ?
    Slash,
    /// . >
    Period,

    /// SDL Keycode
    /// `
    Backquote,
    /// !
    Exclaim,
    /// @
    At,
    /// \"
    Quotedbl,
    /// #
    Hash,
    /// $
    Dollar,
    /// %
    Percent,
    /// ^
    Caret,
    /// &
    Ampersand,
    /// *
    Asterisk,
    /// \'
    Quote,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// _
    Underline,
    /// +
    Plus,
    /// <
    Less,
    /// >
    Greater,
    /// ?
    Question,

    Tab,
    CapsLock,
    Copy,
    Paste,
    Cut,
    VolumeDown,
    VolumeUp,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    //// .
    NumpadDecimal,
    /// ,
    NumpadComma,
    /// enter
    NumpadEnter,
    /// =
    NumpadEquals,
    /// +
    NumpadAdd,
    /// -
    NumpadSubtract,
    /// *
    NumpadMultiply,
    /// /
    NumpadDivide,

    /// winit special code
    Compose,
    AbntC1,
    AbntC2,
    Apps,
    Ax,
    Calculator,
    Capital,
    Convert,
    Kana,
    Kanji,
    Mail,
    MediaSelect,
    MediaStop,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    OEM102,
    PlayPause,
    Power,
    PrevTrack,
    Sleep,
    Stop,
    Sysrq,
    Unlabeled,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,

    ///SDL2 scancode
    Application,
    Execute,
    Help,
    Menu,
    Select,
    Again,
    Undo,
    Find,
    AltErase,
    Sysreq,
    Cancel,
    Clear,
    Return2,
    Separator,
    Out,
    Oper,
    ClearAgain,
    CrSel,
    ExSel,
    ThousandsSeparator,
    DecimalSeparator,
    CurrencyUnit,
    CurrencySubUnit,
    LGui,
    RGui,
    Mode,
    AudioNext,
    AudioPrev,
    AudioStop,
    AudioPlay,
    AudioMute,
    Www,
    Computer,
    AcSearch,
    AcHome,
    AcBack,
    AcForward,
    AcStop,
    AcRefresh,
    AcBookmarks,
    BrightnessDown,
    BrightnessUp,
    DisplaySwitch,
    Eject,

    Kp00,
    Kp000,
    KpEqualsAS400,
    KbdIllumToggle,
    KbdIllumDown,
    KbdIllumUp,
    KpLeftParen,
    KpRightParen,
    KpLeftBrace,
    KpRightBrace,
    KpA,
    KpB,
    KpC,
    KpD,
    KpE,
    KpF,
    KpXor,
    KpPower,
    KpPercent,
    KpDblAmpersand,
    KpVerticalBar,
    KpDblVerticalBar,
    KpMemStore,
    KpMemRecall,
    KpMemClear,
    KpMemAdd,
    KpMemSubtract,
    KpMemMultiply,
    KpMemDivide,
    KpPlusMinus,
    KpClearEntry,
    KpBinary,
    KpOctal,
    KpHexadecimal,
}
