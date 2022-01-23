use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};
use std::path::Path;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowBuilder};
use sdl2::{EventPump, EventSubsystem};

use crate::device::{DisplayWindow, GPUContext};
use crate::graphic::base::*;
use crate::widget::*;

/// 事件上下文
#[allow(missing_debug_implementations)]
pub struct WEventContext<M: 'static> {
    /// 窗口id
    window: Window,
    /// 鼠标位置
    cursor_pos: Point<f32>,
    /// 窗口事件
    window_event: Option<Event>,
    /// 自定义事件
    message: Option<M>,
    /// 自定义事件广播器
    message_channel: EventSubsystem,
}

impl<M: 'static> WEventContext<M> {
    pub fn new(window: Window, event_channel: EventSubsystem, type_id: M) -> WEventContext<M> {
        event_channel.register_custom_event::<M>();
        WEventContext {
            window,
            cursor_pos: Point::new(-1.0, -1.0),
            window_event: None,
            message: None,
            message_channel: event_channel,
        }
    }

    /// 更新鼠标坐标
    pub fn set_cursor_pos<P: Into<Point<f32>>>(&mut self, pos: P) {
        self.cursor_pos = pos.into();
    }

    pub fn get_cursor_pos(&self) -> Point<f32> {
        self.cursor_pos
    }

    /// 设置鼠标图标
    pub fn set_cursor_icon(&mut self, cursor: Cursor) {}
    /// 设置输入框位置
    pub fn set_ime_position(&mut self) {}
    /// 获取当前事件
    // pub fn get_event(&self) -> GEvent {
    //     self.window_event.as_ref().unwrap().into()
    // }

    pub fn get_message(&self) -> Option<&M> {
        self.message.as_ref()
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }
    // /// 发送自定义事件消息
    // pub fn send_message(&self, message: M) {
    //     self.message_channel.send_event(message).ok();
    // }
}

/// 初始化窗口
#[cfg(feature = "sdl2_impl")]
pub(crate) async fn init<'a, M: 'static + Debug + Default>(
    setting: Setting,
) -> DisplayWindow<'a, M> {
    log::info!("Initializing the window...");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_size = Point::new(setting.size.x as u32, setting.size.y as u32);
    let mut builder = video_subsystem
        .window(setting.title.as_str(), window_size.x, window_size.y)
        .position_centered()
        .resizable();
    let window = builder.build().map_err(|e| e.to_string()).unwrap();
    let channel = sdl_context.event().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let gpu_context = GPUContext::new(&window, window_size).await;
    let event_context = WEventContext::new(window, channel, M::default());
    let font_map = GCharMap::new(setting.font_path, DEFAULT_FONT_SIZE);
    let display_window = DisplayWindow {
        gpu_context,
        event_loop: event_pump,
        event_context,
        font_map,
    };
    return display_window;
}
