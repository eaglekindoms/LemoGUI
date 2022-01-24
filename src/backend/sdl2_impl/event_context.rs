use std::fmt::Debug;

use futures::channel::mpsc;
use futures::{task, StreamExt};
use sdl2::event::{Event, WindowEvent};
use sdl2::video::Window;
use sdl2::{EventPump, EventSubsystem};

use crate::device::{DisplayWindow, GPUContext};
use crate::graphic::base::*;
use crate::graphic::style::Style;
use crate::widget::*;

/// 事件上下文
#[allow(missing_debug_implementations)]
pub struct SEventContext<M: 'static> {
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

pub struct SEventListener<M: 'static> {
    pub event_pump: EventPump,
    message: Option<M>,
}

impl<M: 'static> SEventContext<M> {
    pub fn new(window: Window, event_channel: EventSubsystem) -> SEventContext<M> {
        event_channel.register_custom_event::<M>();
        SEventContext {
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
    pub fn get_event(&self) -> GEvent {
        self.window_event.clone().unwrap().into()
    }

    pub fn get_message(&self) -> Option<&M> {
        self.message.as_ref()
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }
    /// 发送自定义事件消息
    pub fn send_message(&self, message: M) {
        // self.message_channel.send_event(message).ok();
    }

    /// 键鼠单击动画效果
    pub fn action_animation(
        &self,
        style: &mut Style,
        position: &Rectangle,
        message: Option<M>,
    ) -> bool {
        false
    }
}

/// 初始化窗口
pub(crate) async fn init<M: 'static + Debug>(setting: Setting) -> DisplayWindow<M> {
    log::info!("Initializing the window...");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_size = Point::new(setting.size.x as u32, setting.size.y as u32);

    let window = video_subsystem
        .window(setting.title.as_str(), window_size.x, window_size.y)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let channel = sdl_context.event().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let gpu_context = GPUContext::new(&window, window_size).await;
    let event_context: SEventContext<M> = SEventContext::new(window, channel);
    let font_map = GCharMap::new(setting.font_path, DEFAULT_FONT_SIZE);
    let display_window = DisplayWindow {
        gpu_context,
        event_loop: SEventListener::<M> {
            event_pump,
            message: None,
        },
        event_context,
        font_map,
    };
    return display_window;
}

/// 运行窗口实例
pub(crate) fn run<C, M>(window: DisplayWindow<M>, container: C)
where
    C: ComponentModel<M> + 'static,
    M: 'static + Debug,
{
    let mut event_pump: EventPump = window.event_loop.event_pump;
    'running: loop {
        for event in event_pump.poll_iter() {}
    }
}

/// 事件监听方法
async fn event_listener<C, M>(
    mut gpu_context: GPUContext,
    mut event_context: SEventContext<M>,
    mut font_map: GCharMap,
    mut container: C,
    mut receiver: mpsc::UnboundedReceiver<sdl2::event::Event>,
) where
    C: ComponentModel<M> + 'static,
    M: 'static + Debug,
{
    while let Some(event) = receiver.next().await {
        match event {
            Event::Window {
                window_id,
                win_event: WindowEvent::SizeChanged(width, height),
                ..
            } if window_id == event_context.window.id() => {
                let new_size = Point::new(width as u32, height as u32);
                gpu_context.update_surface_configure(new_size);
            }
            Event::Quit { .. } => {
                break;
            }
            _ => {}
        }
    }
}
