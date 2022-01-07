use std::fmt::Debug;
use std::future::Future;
use std::path::Path;

use futures::{StreamExt, task};
use futures::channel::mpsc;
use winit::event::*;
use winit::event_loop::*;
use winit::window::*;

use crate::device::{Container, DisplayWindow, GPUContext};
use crate::graphic::base::*;
use crate::graphic::style::Style;
use crate::widget::{Component, EventType, GEvent, Setting, State};

/// 事件上下文
pub struct WEventContext<'a, M: 'static> {
    /// 窗口id
    pub window: Window,
    /// 鼠标位置
    pub cursor_pos: Point<f32>,
    /// 窗口事件
    pub window_event: Option<WindowEvent<'a>>,
    /// 自定义事件
    pub message: Option<M>,
    /// 自定义事件广播器
    message_channel: EventLoopProxy<M>,
}

impl<'a, M: 'static> WEventContext<'a, M> {
    pub fn new(window: Window, event_loop: &EventLoop<M>) -> WEventContext<'a, M> {
        WEventContext {
            window,
            cursor_pos: Point::new(-1.0, -1.0),
            window_event: None,
            message: None,
            message_channel: event_loop.create_proxy(),
        }
    }

    /// 更新鼠标坐标
    pub fn update_cursor<P: Into<Point<f32>>>(&mut self, pos: P) {
        self.cursor_pos = pos.into();
    }

    /// 获取当前事件
    pub fn get_event(&self) -> GEvent {
        self.window_event.as_ref().unwrap().into()
    }

    /// 发送自定义事件消息
    pub fn send_message(&self, message: M) {
        self.message_channel.send_event(message).ok();
    }

    /// 键鼠单击动画效果
    pub fn action_animation(&self, style: &mut Style, position: &Rectangle,
                            message: Option<M>) -> bool {
        let input = position
            .contain_coord(self.cursor_pos);
        if input && message.is_some() {
            let message = message.unwrap();
            let hover_color = style.get_hover_color();
            let back_color = style.get_back_color();
            if self.get_event().state == State::Pressed {
                style.display_color(hover_color);
                self.send_message(message);
            } else if self.get_event().state == State::Released {
                style.display_color(back_color);
            }
            return true;
        }
        return false;
    }
}

impl<'a, M: 'static> WEventContext<'a, M> {
    /// 事件监听器
    /// 作用：监听用户交互事件
    pub fn component_listener(&self, listener: &mut Component<M>) -> bool
    {
        let mut key_listener = false;
        let mut mouse_listener = false;
        let hover_listener;
        let g_event = self.get_event();
        match g_event.event {
            EventType::Mouse(mouse) => {
                if g_event.state == State::Released {
                    self.window.set_ime_position(self.cursor_pos);
                }
                mouse_listener = listener.widget.action_listener(&self, mouse);
            }
            EventType::KeyBoard(key_code) => {
                key_listener = listener.widget.key_listener(&self, key_code);
            }
            EventType::ReceivedCharacter(c) => {
                listener.widget.received_character(&self, c);
            }
            _ => {}
        }
        hover_listener = listener.widget.hover_listener(&self);
        key_listener || mouse_listener || hover_listener
    }
}

/// 初始化窗口
pub(crate) async fn init<'a, M: 'static + Debug>(setting: Setting) -> DisplayWindow<'a, M>
{
    log::info!("Initializing the window...");
    let mut builder = WindowBuilder::new();
    let icon = if setting.icon_path.is_some() {
        load_icon(Path::new(setting.icon_path.unwrap().as_str()))
    } else {
        None
    };
    builder = builder.with_title(setting.title)
        .with_inner_size(winit::dpi::LogicalSize::new(setting.size.x, setting.size.y))
        .with_window_icon(icon);
    let event_loop = EventLoop::<M>::with_user_event();
    let window = builder.build(&event_loop).unwrap();
    let gpu_context = GPUContext::new(&window, window.inner_size().into()).await;
    let event_context = WEventContext::new(window, &event_loop);
    let display_window = DisplayWindow {
        gpu_context,
        event_loop,
        event_context,
    };
    return display_window;
}

/// 运行窗口实例
pub(crate) fn run<C, M>(window: DisplayWindow<'static, M>, container: C)
    where C: Container<M> + 'static, M: 'static + Debug {
    let (mut sender, receiver)
        = mpsc::unbounded();
    let mut instance_listener
        = Box::pin(event_listener(window.gpu_context,
                                  window.event_context,
                                  container,
                                  receiver));
    let mut context = task::Context::from_waker(task::noop_waker_ref());
    window.event_loop.run(move |event, _, control_flow| {
        if let ControlFlow::Exit = control_flow {
            return;
        }
        // 封装窗口尺寸变更事件
        let event = match event {
            Event::WindowEvent {
                event:
                WindowEvent::ScaleFactorChanged {
                    new_inner_size,
                    ..
                },
                window_id,
            } => Some(Event::WindowEvent {
                event: WindowEvent::Resized(*new_inner_size),
                window_id,
            }),
            _ => event.to_static(),
        };
        // 异步发送到事件监听器
        if let Some(event) = event {
            sender.start_send(event).expect("Send event");
            let poll = instance_listener.as_mut().poll(&mut context);
            *control_flow = match poll {
                task::Poll::Pending => {
                    // println!("--------pending--------");
                    ControlFlow::Wait
                }
                task::Poll::Ready(_) => {
                    // println!("--------ready--------");
                    ControlFlow::Exit
                }
            };
        }
    });
}

/// 事件监听方法
async fn event_listener<C, M>(mut gpu_context: GPUContext,
                              mut event_context: WEventContext<'_, M>,
                              mut container: C,
                              mut receiver: mpsc::UnboundedReceiver<winit::event::Event<'_, M>>)
    where C: Container<M> + 'static, M: 'static + Debug
{
    while let Some(event) = receiver.next().await {
        match event {
            Event::WindowEvent {
                event,
                window_id,
            } if window_id == event_context.window.id() => {
                // 捕获窗口关闭请求
                if event == WindowEvent::CloseRequested {
                    break;
                }
                match event {
                    WindowEvent::Resized(new_size) => {
                        // 更新swapChain交换缓冲区
                        gpu_context.update_surface_configure(new_size);
                    }
                    // 储存鼠标位置坐标
                    WindowEvent::CursorMoved { position, .. }
                    => {
                        event_context.update_cursor(position);
                    }
                    _ => {}
                }
                // 监听到组件关注事件，决定是否重绘
                event_context.window_event = Some(event);
                if container.update(&mut event_context) {
                    gpu_context.present(&mut container)
                }
            }
            Event::RedrawRequested(window_id)
            if window_id == event_context.window.id() => {
                gpu_context.present(&mut container)
            }
            Event::UserEvent(event) => {
                event_context.message = Some(event);
            }
            _ => {}
        }
    };
}

/// 加载icon
fn load_icon(path: &Path) -> Option<Icon> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Some(Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon"))
}