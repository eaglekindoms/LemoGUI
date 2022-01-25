use std::fmt::Debug;
use std::future::Future;
use std::path::Path;

use futures::channel::mpsc;
use futures::{task, StreamExt};
use winit::event::*;
use winit::event_loop::*;
use winit::window::*;

use crate::adapter::*;
use crate::event::*;
use crate::graphic::base::*;
use crate::instance::Setting;
use crate::widget::*;

/// 事件上下文
pub struct WEventContext<M: 'static> {
    /// 窗口id
    window: Window,
    /// 鼠标位置
    cursor_pos: Point<f32>,
    /// 窗口事件
    window_event: Option<GEvent>,
    /// 自定义事件
    message: Option<M>,
    /// 自定义事件广播器
    message_channel: EventLoopProxy<M>,
}

impl<M: 'static> WEventContext<M> {
    pub fn new(window: Window, event_loop: &EventLoop<M>) -> WEventContext<M> {
        WEventContext {
            window,
            cursor_pos: Point::new(-1.0, -1.0),
            window_event: None,
            message: None,
            message_channel: event_loop.create_proxy(),
        }
    }
}

impl<M> EventContext<M> for WEventContext<M> {
    fn get_window_id(&self) -> String {
        format!("{:?}", &self.window.id())
    }

    /// 更新鼠标坐标
    fn set_cursor_pos(&mut self, pos: Point<f32>) {
        self.cursor_pos = pos;
    }

    fn get_cursor_pos(&self) -> Point<f32> {
        self.cursor_pos
    }

    /// 设置鼠标图标
    fn set_cursor_icon(&mut self, cursor: Cursor) {
        match cursor {
            Cursor::Default => self.window.set_cursor_icon(CursorIcon::Default),
            Cursor::Text => self.window.set_cursor_icon(CursorIcon::Text),
        }
    }
    /// 设置输入框位置
    fn set_ime_position(&mut self) {
        self.window.set_ime_position(self.cursor_pos);
    }

    fn set_event(&mut self, event: GEvent) {
        self.window_event = Some(event);
    }

    /// 获取当前事件
    fn get_event(&self) -> GEvent {
        self.window_event.clone().unwrap()
    }

    fn get_message(&self) -> Option<&M> {
        self.message.as_ref()
    }

    fn set_message(&mut self, message: Option<M>) {
        self.message = message;
    }
    /// 发送自定义事件消息
    fn send_message(&self, message: M) {
        self.message_channel.send_event(message).ok();
    }
}

/// 初始化窗口
pub(crate) async fn init<M: 'static + Debug>(setting: Setting) -> DisplayWindow<M> {
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
        event_context: Box::new(event_context),
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
#[cfg(feature = "winit_impl")]
async fn event_listener<C, M>(
    mut gpu_context: GPUContext,
    mut event_context: Box<dyn EventContext<M>>,
    mut font_map: GCharMap,
    mut container: C,
    mut receiver: mpsc::UnboundedReceiver<winit::event::Event<'static, M>>,
) where
    C: ComponentModel<M> + 'static,
    M: 'static + Debug,
{
    while let Some(event) = receiver.next().await {
        match event {
            Event::WindowEvent { event, window_id }
                if format!("{:?}", window_id)
                    .eq_ignore_ascii_case(event_context.get_window_id().as_str()) =>
            {
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
                        event_context.set_cursor_pos(position.into());
                    }
                    _ => {}
                }
                // 监听到组件关注事件，决定是否重绘
                event_context.set_event(event.into());
                if container.listener(&mut *event_context) {
                    gpu_context.present(&mut container, &mut font_map)
                }
            }
            Event::RedrawRequested(window_id)
                if format!("{:?}", window_id)
                    .eq_ignore_ascii_case(event_context.get_window_id().as_str()) =>
            {
                gpu_context.present(&mut container, &mut font_map)
            }
            Event::UserEvent(event) => {
                event_context.set_message(Some(event));
                println!("{:?}", event_context.get_message());
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
