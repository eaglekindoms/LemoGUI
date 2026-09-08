use std::fmt::Debug;

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
}

impl<M: 'static> SEventContext<M> {
    pub fn new(window: Window, event_channel: EventSubsystem) -> SEventContext<M> {
        let _ = event_channel.register_custom_event::<M>();
        SEventContext {
            window,
            cursor_pos: Point::new(-1.0, -1.0),
            window_event: None,
            message: None,
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
    fn set_ime_position(&mut self, pos: Point<f32>, height: f32) {
        let text_input = self.window.subsystem().text_input();
        if !text_input.is_active() {
            text_input.start();
        }
        let h = height.max(16.0) as u32;
        text_input.set_rect(sdl2::rect::Rect::new(pos.x as i32, pos.y as i32, 2, h));
    }

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
    fn send_message(&mut self, message: M) {
        self.message = Some(message);
    }
}

/// 初始化窗口
pub(crate) async fn init<M: 'static + Debug>(setting: Setting) -> DisplayWindow<M> {
    log::info!("Initializing the window...");
    // 必须在 SDL_Init 之前：否则系统 IME 候选框被隐藏
    sdl2::hint::set("SDL_IME_SHOW_UI", "1");
    sdl2::hint::set("SDL_IME_INTERNAL_EDITING", "0");
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
    let gpu_context = {
        use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
        let instance = GPUContext::create_instance();
        // SAFETY: window 在整个程序生命周期内有效，surface 不会超过 window 的生命周期
        let target = wgpu::SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: Some(window.display_handle().unwrap().as_raw()),
            raw_window_handle: window.window_handle().unwrap().as_raw(),
        };
        let surface = unsafe { instance.create_surface_unsafe(target).unwrap() };
        GPUContext::new(surface, &instance, window_size).await
    };
    let event_context: SEventContext<M> = SEventContext::new(window, channel);
    event_context.window.subsystem().text_input().start();
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
pub(crate) fn run<C, M>(window: DisplayWindow<M>, mut container: C)
where
    C: ComponentModel<M> + 'static,
    M: 'static + Debug,
{
    let mut gpu_context = window.gpu_context;
    let mut event_context = window.event_context;
    let mut font_map = window.font_map;
    let mut event_pump: EventPump = window.event_loop;
    gpu_context.present(&mut container, &mut font_map);
    loop {
        let mut events: Vec<Event> = event_pump.poll_iter().collect();
        if events.is_empty() {
            match event_pump.wait_event_timeout(16) {
                Some(event) => {
                    events.push(event);
                    events.extend(event_pump.poll_iter());
                }
                None => continue,
            }
        }
        let mut dirty = false;
        for event in events {
            if dispatch_event(
                event,
                &mut gpu_context,
                &mut event_context,
                &mut container,
            ) {
                dirty = true;
            }
        }
        container.commit();
        if dirty {
            gpu_context.present(&mut container, &mut font_map);
            if let Some((pos, h)) = container.ime_caret() {
                event_context.set_ime_position(pos, h);
            }
        }
    }
}

fn dispatch_event<C, M>(
    event: Event,
    gpu_context: &mut GPUContext,
    event_context: &mut SEventContext<M>,
    container: &mut C,
) -> bool
where
    C: ComponentModel<M> + 'static,
    M: 'static + Debug,
{
    if event.is_user_event() {
        return false;
    }
    if let Some(id) = event.get_window_id() {
        if id != 0 && id != event_context.window.id() {
            return false;
        }
    }
    match event {
        Event::Window { win_event, .. } => match win_event {
            WindowEvent::Resized(width, height) | WindowEvent::SizeChanged(width, height) => {
                let new_size = Point::new(width as u32, height as u32);
                gpu_context.update_surface_configure(new_size);
                true
            }
            WindowEvent::FocusGained => {
                event_context.window.subsystem().text_input().start();
                false
            }
            WindowEvent::Close => {
                println!("----- Close window -----");
                ::std::process::exit(0);
            }
            _ => false,
        },
        Event::Quit { .. } => {
            println!("----- Close window -----");
            ::std::process::exit(0);
        }
        Event::MouseMotion { x, y, .. } => {
            event_context.set_cursor_pos(Point::new(x as f32, y as f32));
            event_context.set_event(GEvent {
                event: EventType::Other,
                state: State::None,
            });
            container.listener(event_context)
        }
        Event::TextInput { text, .. } => {
            let mut dirty = false;
            for c in text.chars() {
                event_context.set_event(GEvent {
                    event: EventType::ReceivedCharacter(c),
                    state: State::None,
                });
                if container.listener(event_context) {
                    dirty = true;
                }
            }
            dirty
        }
        Event::TextEditing { .. } => false,
        Event::MouseButtonDown { .. }
        | Event::MouseButtonUp { .. }
        | Event::KeyUp { .. }
        | Event::KeyDown { .. } => {
            event_context.set_event(event.into());
            container.listener(event_context)
        }
        _ => false,
    }
}
