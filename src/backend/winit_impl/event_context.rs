use std::fmt::Debug;
use std::future::Future;
use std::path::Path;

use futures::channel::mpsc;
use futures::{task, StreamExt};
use winit::event::*;
use winit::event_loop::*;
use winit::window::*;

use crate::device::*;
use crate::graphic::base::*;
use crate::graphic::style::Style;
use crate::widget::*;

/// 事件上下文
pub struct WEventContext<'a, M: 'static> {
    /// 窗口id
    window: Window,
    /// 鼠标位置
    cursor_pos: Point<f32>,
    /// 窗口事件
    window_event: Option<WindowEvent<'a>>,
    /// 自定义事件
    message: Option<M>,
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
    pub fn set_cursor_pos<P: Into<Point<f32>>>(&mut self, pos: P) {
        self.cursor_pos = pos.into();
    }

    pub fn get_cursor_pos(&self) -> Point<f32> {
        self.cursor_pos
    }

    /// 设置鼠标图标
    pub fn set_cursor_icon(&mut self, cursor: Cursor) {
        match cursor {
            Cursor::Default => self.window.set_cursor_icon(CursorIcon::Default),
            Cursor::Text => self.window.set_cursor_icon(CursorIcon::Text),
        }
    }
    /// 设置输入框位置
    pub fn set_ime_position(&mut self) {
        self.window.set_ime_position(self.cursor_pos);
    }
    /// 获取当前事件
    pub fn get_event(&self) -> GEvent {
        self.window_event.as_ref().unwrap().into()
    }

    pub fn get_message(&self) -> Option<&M> {
        self.message.as_ref()
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }
    /// 发送自定义事件消息
    pub fn send_message(&self, message: M) {
        self.message_channel.send_event(message).ok();
    }

    /// 键鼠单击动画效果
    pub fn action_animation(
        &self,
        style: &mut Style,
        position: &Rectangle,
        message: Option<M>,
    ) -> bool {
        let input = position.contain_coord(self.cursor_pos);
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

/// 初始化窗口
pub(crate) async fn init<'a, M: 'static + Debug>(setting: Setting) -> DisplayWindow<'a, M> {
    log::info!("Initializing the window...");
    let mut builder = WindowBuilder::new();
    let icon = if setting.icon_path.is_some() {
        load_icon(Path::new(setting.icon_path.unwrap().as_str()))
    } else {
        None
    };
    builder = builder
        .with_title(setting.title)
        .with_inner_size(winit::dpi::LogicalSize::new(setting.size.x, setting.size.y))
        .with_window_icon(icon);
    let event_loop = EventLoop::<M>::with_user_event();
    let window = builder.build(&event_loop).unwrap();
    let gpu_context = GPUContext::new(&window, window.inner_size().into()).await;
    let event_context = WEventContext::new(window, &event_loop);
    let font_map = GCharMap::new(setting.font_path, DEFAULT_FONT_SIZE);
    let display_window = DisplayWindow {
        gpu_context,
        event_loop,
        event_context,
        font_map,
    };
    return display_window;
}

/// 运行窗口实例
pub(crate) fn run<C, M>(window: DisplayWindow<'static, M>, container: C)
where
    C: ComponentModel<M> + 'static,
    M: 'static + Debug,
{
    let (mut sender, receiver) = mpsc::unbounded();
    let mut instance_listener = Box::pin(event_listener(
        window.gpu_context,
        window.event_context,
        window.font_map,
        container,
        receiver,
    ));
    let mut context = task::Context::from_waker(task::noop_waker_ref());
    window.event_loop.run(move |event, _, control_flow| {
        if let ControlFlow::Exit = control_flow {
            return;
        }
        // 封装窗口尺寸变更事件
        let event = match event {
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
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
async fn event_listener<C, M>(
    mut gpu_context: GPUContext,
    mut event_context: WEventContext<'_, M>,
    mut font_map: GCharMap,
    mut container: C,
    mut receiver: mpsc::UnboundedReceiver<winit::event::Event<'_, M>>,
) where
    C: ComponentModel<M> + 'static,
    M: 'static + Debug,
{
    while let Some(event) = receiver.next().await {
        match event {
            Event::WindowEvent { event, window_id } if window_id == event_context.window.id() => {
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
                    WindowEvent::CursorMoved { position, .. } => {
                        event_context.set_cursor_pos(position);
                    }
                    _ => {}
                }
                // 监听到组件关注事件，决定是否重绘
                event_context.window_event = Some(event);
                if container.listener(&mut event_context) {
                    gpu_context.present(&mut container, &mut font_map)
                }
            }
            Event::RedrawRequested(window_id) if window_id == event_context.window.id() => {
                gpu_context.present(&mut container, &mut font_map)
            }
            Event::UserEvent(event) => {
                event_context.message = Some(event);
            }
            _ => {}
        }
    }
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
