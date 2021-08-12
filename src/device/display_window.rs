use std::fmt::Debug;
use std::future::Future;

use futures::{StreamExt, task};
use futures::channel::mpsc;
use log::Log;
use winit::event::*;
use winit::event_loop::*;
use winit::window::*;

use crate::device::container::Container;
use crate::device::event_context::ELContext;
use crate::device::wgpu_context::WGContext;
use crate::graphic::base::shape::Point;
use crate::widget::message::Message;

/// 窗口结构体
/// 作用：封装窗体，事件循环器，图形上下文
pub struct DisplayWindow<'a, M: 'static> {
    /// 图形上下文
    pub wgcontext: WGContext,
    /// 事件上下文
    pub event_context: Option<ELContext<'a, M>>,
}

/// 装填组件容器，启动窗口
pub fn start<C, M>(builder: WindowBuilder, build_container: &Fn(WGContext) -> C)
    where C: Container<M> + 'static, M: 'static + Debug
{
    use futures::executor::block_on;
    block_on(init::<C, M>(builder, build_container));
    log::info!("Initializing the example...");
}

/// 初始化窗口
async fn init<C, M>(builder: WindowBuilder, build_container: &Fn(WGContext) -> C)
    where C: Container<M> + 'static, M: 'static + Debug {
    log::info!("Initializing the window...");
    let event_loop = EventLoop::<M>::with_user_event();
    let window = builder.build(&event_loop).unwrap();

    let wgcontext = WGContext::new(&window);

    let container = build_container(wgcontext.await);

    let el_context = ELContext {
        window_id: window.id(),
        cursor_pos: None,
        window_event: None,
        custom_event: None,
        event_loop_proxy: event_loop.create_proxy(),
    };

    let (mut sender, receiver) = mpsc::unbounded();
    let mut instance = Box::pin(event_listener(el_context, container, receiver));
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
            let poll = instance.as_mut().poll(&mut context);
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
async fn event_listener<C, M>(mut el_context: ELContext<'_, M>,
                              mut container: C,
                              mut receiver: mpsc::UnboundedReceiver<winit::event::Event<'_, M>>)
    where C: Container<M> + 'static, M: 'static + Debug
{
    while let Some(event) = receiver.next().await {
        // log::info!("{:#?}", event);
        match event {
            Event::WindowEvent {
                event,
                window_id,
            } if window_id == el_context.window_id => {
                // 监听到组件关注事件，决定是否重绘
                el_context.window_event = Some(event);
                if container.input(&el_context) {
                    container.render();
                }
                match el_context.window_event.as_ref().unwrap() {
                    // 捕获窗口关闭请求
                    WindowEvent::CloseRequested =>
                        break,
                    // 储存鼠标位置
                    WindowEvent::CursorMoved { position, .. }
                    => {
                        el_context.cursor_pos = Some(Point::new(position.x as f32, position.y as f32));
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id)
            if window_id == el_context.window_id => {
                container.render();
            }
            Event::UserEvent(event) => {
                el_context.custom_event = Some(event);
            },

            // Event::MainEventsCleared => {
            //     // RedrawRequested will only trigger once, unless we manually
            //     // request it.
            //     display_device.window.request_redraw();
            // }
            _ => {}
        }
    };
}