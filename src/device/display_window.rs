use std::fmt::Debug;
use std::future::Future;

use futures::{StreamExt, task};
use futures::channel::mpsc;
use winit::event::*;
use winit::event_loop::*;
use winit::window::*;

use crate::device::container::Container;
use crate::device::event_context::EventContext;
use crate::device::wgpu_context::GPUContext;
use crate::graphic::render_middle::PipelineState;

/// 窗口结构体
/// 作用：封装窗体，事件循环器，图形上下文
pub struct DisplayWindow<'a, M: 'static> {
    /// 图形上下文
    pub gpu_context: GPUContext,
    /// 渲染管道
    pub glob_pipeline: PipelineState,
    /// 时间监听器
    event_loop: EventLoop<M>,
    /// 事件上下文
    event_context: EventContext<'a, M>,
}

impl<M: 'static + Debug> DisplayWindow<'static, M> {
    pub fn start<C>(self, mut container: C)
        where C: Container<M> + 'static {
        run_instance(self, container);
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
        let gpu_context = GPUContext::new(&window).await;

        let event_context = EventContext::new(window, &event_loop);
        let glob_pipeline = PipelineState::default(&gpu_context.device);
        let display_window = DisplayWindow {
            gpu_context,
            glob_pipeline,
            event_loop,
            event_context,
        };
        return display_window;
    }
}

/// 运行窗口实例
fn run_instance<C, M>(window: DisplayWindow<'static, M>, container: C)
    where C: Container<M> + 'static, M: 'static + Debug {
    let (mut sender, receiver)
        = mpsc::unbounded();
    let mut instance_listener
        = Box::pin(event_listener(window.gpu_context,
                                  window.glob_pipeline,
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
                              glob_pipeline: PipelineState,
                              mut event_context: EventContext<'_, M>,
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
                    gpu_context.present(&glob_pipeline, &mut container)
                }
            }
            Event::RedrawRequested(window_id)
            if window_id == event_context.window.id() => {
                gpu_context.present(&glob_pipeline, &mut container)
            }
            Event::UserEvent(event) => {
                event_context.message = Some(event);
            }
            _ => {}
        }
    };
}
