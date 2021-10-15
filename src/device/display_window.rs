use std::fmt::Debug;
use std::future::Future;
use std::path::Path;

use futures::{StreamExt, task};
use futures::channel::mpsc;
use winit::event::*;
use winit::event_loop::*;
use winit::window::*;

use crate::device::container::Container;
use crate::device::event_context::ELContext;
use crate::device::wgpu_context::WGContext;
use crate::widget::Instance;

/// 窗口结构体
/// 作用：封装窗体，事件循环器，图形上下文
pub struct DisplayWindow<'a, M: 'static> {
    /// 图形上下文
    pub wgcontext: WGContext,
    /// 时间监听器
    event_loop: EventLoop<M>,
    /// 事件上下文
    event_context: ELContext<'a, M>,
}

impl<M: 'static + Debug> DisplayWindow<'static, M> {
    pub fn start<C>(self, mut container: C, instance: impl Instance<M=M> + 'static) where C: Container<M> + 'static {
        container.add_comp(&instance);
        run_instance(self.event_loop, self.wgcontext, container, instance, self.event_context);
    }

    pub fn request_container<C>(&self) -> C where C: Container<M> + 'static {
        C::new(&self.wgcontext)
    }
    pub fn new<'a>(builder: WindowBuilder) -> DisplayWindow<'a, M> {
        use futures::executor::block_on;
        block_on(Self::init_window(builder))
    }
    /// 初始化窗口
    async fn init_window<'a>(builder: WindowBuilder) -> DisplayWindow<'a, M>
    {
        log::info!("Initializing the window...");
        let event_loop = EventLoop::<M>::with_user_event();
        let window = builder.build(&event_loop).unwrap();
        let wgcontext = WGContext::new(&window).await;

        let el_context = ELContext {
            window,
            cursor_pos: None,
            window_event: None,
            message: None,
            message_channel: event_loop.create_proxy(),
        };
        let display_window = DisplayWindow {
            wgcontext,
            event_loop,
            event_context: el_context,
        };
        return display_window;
    }
}

/// 运行窗口实例
fn run_instance<C, M>(event_loop: EventLoop<M>, wgcontext: WGContext,
                      container: C, instance: impl Instance<M=M> + 'static, el_context: ELContext<'static, M>)
    where C: Container<M> + 'static, M: 'static + Debug {
    let (mut sender, receiver)
        = mpsc::unbounded();
    let mut instance_listener
        = Box::pin(event_listener(wgcontext, el_context, container, instance, receiver));
    let mut context = task::Context::from_waker(task::noop_waker_ref());
    event_loop.run(move |event, _, control_flow| {
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
                    // log::info!("pending");
                    ControlFlow::Wait
                }
                task::Poll::Ready(_) => {
                    // log::info!("--------ready--------");
                    ControlFlow::Exit
                }
            };
        }
    });
}

/// 事件监听方法
async fn event_listener<C, M>(mut wgcontext: WGContext,
                              mut el_context: ELContext<'_, M>,
                              mut container: C,
                              mut instance: impl Instance<M=M>,
                              mut receiver: mpsc::UnboundedReceiver<winit::event::Event<'_, M>>)
    where C: Container<M> + 'static, M: 'static + Debug
{
    while let Some(event) = receiver.next().await {
        // log::info!("{:#?}", event);
        match event {
            Event::WindowEvent {
                event,
                window_id,
            } if window_id == el_context.window.id() => {
                // 捕获窗口关闭请求
                if event == WindowEvent::CloseRequested {
                    break;
                }
                // 监听到组件关注事件，决定是否重绘
                el_context.window_event = Some(event);
                if container.input(&mut wgcontext, &mut el_context, &mut instance) {
                    container.render(&mut wgcontext);
                }
            }
            Event::RedrawRequested(window_id)
            if window_id == el_context.window.id() => {
                container.render(&mut wgcontext);
            }
            Event::UserEvent(event) => {
                el_context.message = Some(event);
            }
            _ => {}
        }
    };
}

/// 加载icon
pub fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
