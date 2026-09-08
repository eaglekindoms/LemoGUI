use std::fmt::Debug;
use std::path::Path;
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::*;
use winit::event_loop::*;
use winit::window::*;

use crate::adapter::*;
use crate::event;
use crate::event::*;
use crate::graphic::base::*;
use crate::instance::Setting;
use crate::widget::*;

/// 事件上下文
pub struct WEventContext<M: 'static> {
    /// 窗口
    pub window: Arc<Window>,
    /// 鼠标位置
    cursor_pos: Point<f32>,
    /// 窗口事件
    window_event: Option<GEvent>,
    /// 自定义事件
    message: Option<M>,
    /// 保留代理以免改 EventLoop 泛型；消息已同帧写入 message
    #[allow(dead_code)]
    message_channel: EventLoopProxy<M>,
}

impl<M: 'static> WEventContext<M> {
    pub fn new(window: Arc<Window>, event_loop: &EventLoop<M>) -> WEventContext<M> {
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
    fn set_cursor_pos(&mut self, pos: Point<f32>) {
        self.cursor_pos = pos;
    }

    fn get_cursor_pos(&self) -> Point<f32> {
        self.cursor_pos
    }

    fn set_cursor_icon(&mut self, cursor: event::Cursor) {
        match cursor {
            event::Cursor::Default => self.window.set_cursor(CursorIcon::Default),
            event::Cursor::Text => self.window.set_cursor(CursorIcon::Text),
        }
    }

    fn set_ime_position(&mut self, pos: Point<f32>, height: f32) {
        self.window.set_ime_allowed(true);
        let h = height.max(16.0) as u32;
        self.window.set_ime_cursor_area(
            winit::dpi::PhysicalPosition::new(pos.x as i32, pos.y as i32),
            winit::dpi::PhysicalSize::new(2, h),
        );
    }

    fn set_event(&mut self, event: GEvent) {
        self.window_event = Some(event);
    }

    fn get_event(&self) -> GEvent {
        self.window_event.clone().unwrap()
    }

    fn get_message(&self) -> Option<&M> {
        self.message.as_ref()
    }

    fn set_message(&mut self, message: Option<M>) {
        self.message = message;
    }

    fn send_message(&mut self, message: M) {
        self.message = Some(message);
    }
}

/// 应用状态结构体
struct App<C, M: 'static> {
    gpu_context: GPUContext,
    event_context: WEventContext<M>,
    font_map: GCharMap,
    container: C,
}

impl<C, M> ApplicationHandler<M> for App<C, M>
where
    C: ComponentModel<M> + 'static,
    M: 'static + Debug,
{
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        self.event_context.window.request_redraw();
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: M) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if window_id != self.event_context.window.id() {
            return;
        }

        if event == WindowEvent::CloseRequested {
            event_loop.exit();
            return;
        }

        match &event {
            WindowEvent::Resized(new_size) => {
                let size = Point::new(new_size.width, new_size.height);
                self.gpu_context.update_surface_configure(size);
                self.event_context.window.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.event_context
                    .set_cursor_pos(Point::new(position.x as f32, position.y as f32));
            }
            WindowEvent::RedrawRequested => {
                self.container.commit();
                self.event_context.set_event(GEvent {
                    event: EventType::Other,
                    state: State::None,
                });
                self.container.listener(&mut self.event_context);
                self.gpu_context
                    .present(&mut self.container, &mut self.font_map);
                if let Some((pos, h)) = self.container.ime_caret() {
                    self.event_context.set_ime_position(pos, h);
                }
                return;
            }
            WindowEvent::Ime(Ime::Enabled) | WindowEvent::Ime(Ime::Preedit(_, _)) => {
                if let Some((pos, h)) = self.container.ime_caret() {
                    self.event_context.set_ime_position(pos, h);
                }
                return;
            }
            WindowEvent::Ime(Ime::Commit(text)) => {
                let text = text.clone();
                let mut dirty = false;
                for c in text.chars() {
                    self.event_context.set_event(GEvent {
                        event: EventType::ReceivedCharacter(c),
                        state: State::None,
                    });
                    if self.container.listener(&mut self.event_context) {
                        dirty = true;
                    }
                }
                if dirty {
                    self.event_context.window.request_redraw();
                }
                return;
            }
            _ => {}
        }

        self.event_context.set_event(event.into());
        if self.container.listener(&mut self.event_context) {
            self.event_context.window.request_redraw();
        }
    }
}

/// 初始化窗口
pub(crate) async fn init<M: 'static + Debug>(setting: Setting) -> DisplayWindow<M> {
    log::info!("Initializing the window...");
    let event_loop = EventLoop::<M>::with_user_event().build().unwrap();

    let icon = if setting.icon_path.is_some() {
        load_icon(Path::new(setting.icon_path.as_ref().unwrap().as_str()))
    } else {
        None
    };

    let window_attributes = Window::default_attributes()
        .with_title(setting.title)
        .with_inner_size(winit::dpi::LogicalSize::new(setting.size.x, setting.size.y))
        .with_window_icon(icon);

    #[allow(deprecated)]
    let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
    window.set_ime_allowed(true);

    let size: Point<u32> = window.inner_size().into();
    let instance = GPUContext::create_instance();
    let surface = instance.create_surface(window.clone()).unwrap();
    let gpu_context = GPUContext::new(surface, &instance, size).await;
    let event_context = WEventContext::new(window, &event_loop);
    let font_map = GCharMap::new(setting.font_path, DEFAULT_FONT_SIZE);
    DisplayWindow {
        gpu_context,
        event_loop,
        event_context,
        font_map,
    }
}

/// 运行窗口实例
pub(crate) fn run<C, M>(window: DisplayWindow<M>, container: C)
where
    C: ComponentModel<M> + 'static,
    M: 'static + Debug,
{
    let mut app = App {
        gpu_context: window.gpu_context,
        event_context: window.event_context,
        font_map: window.font_map,
        container,
    };
    window.event_loop.run_app(&mut app).unwrap();
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
