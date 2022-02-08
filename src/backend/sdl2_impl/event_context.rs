use std::fmt::Debug;

use futures::channel::mpsc;
use futures::{task, Future, StreamExt};
use sdl2::event::{Event, WindowEvent};
use sdl2::video::Window;
use sdl2::{EventPump, EventSubsystem};

use crate::adapter::{DisplayWindow, GPUContext};
use crate::event::*;
use crate::graphic::base::*;
use crate::instance::Setting;
use crate::widget::*;

/// 事件上下文
#[allow(missing_debug_implementations)]
pub struct SEventContext<M: 'static> {
    /// 窗口id
    window: Window,
    /// 鼠标位置
    cursor_pos: Point<f32>,
    /// 窗口事件
    window_event: Option<GEvent>,
    /// 自定义事件
    message: Option<M>,
    /// 自定义事件广播器
    message_channel: EventSubsystem,
}

impl<M: 'static> SEventContext<M> {
    pub fn new(window: Window, event_channel: EventSubsystem) -> SEventContext<M> {
        event_channel.register_custom_event::<M>().unwrap();
        SEventContext {
            window,
            cursor_pos: Point::new(-1.0, -1.0),
            window_event: None,
            message: None,
            message_channel: event_channel,
        }
    }
}

impl<M> EventContext<M> for SEventContext<M> {
    /// 更新鼠标坐标
    fn set_cursor_pos(&mut self, pos: Point<f32>) {
        self.cursor_pos = pos;
    }

    fn get_cursor_pos(&self) -> Point<f32> {
        self.cursor_pos
    }

    /// 设置鼠标图标
    fn set_cursor_icon(&mut self, _cursor: Cursor) {}
    /// 设置输入框位置
    fn set_ime_position(&mut self) {}

    fn set_event(&mut self, event: GEvent) {
        self.window_event = Some(event)
    }

    /// 获取当前事件
    fn get_event(&self) -> GEvent {
        return if let Some(event) = self.window_event.clone() {
            event
        } else {
            GEvent {
                event: EventType::Other,
                state: State::None,
            }
        };
    }

    fn get_message(&self) -> Option<&M> {
        self.message.as_ref()
    }

    fn set_message(&mut self, message: Option<M>) {
        self.message = message;
    }
    /// 发送自定义事件消息
    fn send_message(&self, message: M) {
        self.message_channel.push_custom_event(message).unwrap();
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
    let event_pump = sdl_context.event_pump().unwrap();
    let gpu_context = GPUContext::new(&window, window_size).await;
    let event_context: SEventContext<M> = SEventContext::new(window, channel);
    let font_map = GCharMap::new(setting.font_path, DEFAULT_FONT_SIZE);
    let display_window = DisplayWindow {
        gpu_context,
        event_loop: event_pump,
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
    let (sender, receiver) = mpsc::unbounded();
    let mut instance_listener = Box::pin(event_listener(
        window.gpu_context,
        window.event_context,
        window.font_map,
        container,
        receiver,
    ));
    let mut context = task::Context::from_waker(task::noop_waker_ref());
    let mut event_pump: EventPump = window.event_loop;
    loop {
        for event in event_pump.poll_iter() {
            sender.unbounded_send(event).unwrap();
            let poll = instance_listener.as_mut().poll(&mut context);
            match poll {
                task::Poll::Pending => {
                    // println!("--------pending--------");
                }
                task::Poll::Ready(_) => {
                    // println!("--------ready--------");
                }
            };
        }
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
        if event.is_user_event() {
            event_context.set_message(event.as_user_event_type::<M>());
            log::debug!("customer event: {:?}", event_context.get_message());
        }
        if event.get_window_id() == Some(event_context.window.id()) {
            match event {
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height)
                    | WindowEvent::SizeChanged(width, height) => {
                        let new_size = Point::new(width as u32, height as u32);
                        gpu_context.update_surface_configure(new_size);
                    }
                    WindowEvent::Close => {
                        println!("----- Close window -----");
                        ::std::process::exit(0);
                    }
                    _ => gpu_context.present(&mut container, &mut font_map),
                },
                Event::Quit { .. } => {
                    println!("----- Close window -----");
                    ::std::process::exit(0);
                }
                Event::MouseMotion { x, y, .. } => {
                    event_context.set_cursor_pos(Point::new(x as f32, y as f32))
                }
                Event::MouseButtonDown { .. }
                | Event::MouseButtonUp { .. }
                | Event::KeyUp { .. }
                | Event::KeyDown { .. } => {
                    event_context.set_event(event.into());
                    if container.listener(&mut event_context) {
                        gpu_context.present(&mut container, &mut font_map)
                    }
                }
                _ => {}
            }
        }
    }
}
