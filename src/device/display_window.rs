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
use crate::graphic::render_middle::PipelineState;

/// 窗口结构体
/// 作用：封装窗体，事件循环器，图形上下文
pub struct DisplayWindow<'a, M: 'static> {
    /// 图形上下文
    pub wgcontext: WGContext,
    /// 渲染管道
    pub glob_pipeline: PipelineState,
    /// 时间监听器
    event_loop: EventLoop<M>,
    /// 事件上下文
    event_context: ELContext<'a, M>,
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
        let wgcontext = WGContext::new(&window).await;

        let el_context = ELContext::new(window, &event_loop);
        let glob_pipeline = PipelineState::default(&wgcontext.device);
        let display_window = DisplayWindow {
            wgcontext,
            glob_pipeline,
            event_loop,
            event_context: el_context,
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
        = Box::pin(event_listener(window.wgcontext,
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
async fn event_listener<C, M>(mut wgcontext: WGContext,
                              glob_pipeline: PipelineState,
                              mut el_context: ELContext<'_, M>,
                              mut container: C,
                              mut receiver: mpsc::UnboundedReceiver<winit::event::Event<'_, M>>)
    where C: Container<M> + 'static, M: 'static + Debug
{
    while let Some(event) = receiver.next().await {
        match event {
            Event::WindowEvent {
                event,
                window_id,
            } if window_id == el_context.window.id() => {
                // 捕获窗口关闭请求
                if event == WindowEvent::CloseRequested {
                    break;
                }
                match event {
                    WindowEvent::Resized(new_size) => {
                        // 更新swapChain交换缓冲区
                        wgcontext.update_surface_configure(new_size);
                    }
                    // 储存鼠标位置坐标
                    WindowEvent::CursorMoved { position, .. }
                    => {
                        el_context.update_cursor(position);
                    }
                    _ => {}
                }
                // 监听到组件关注事件，决定是否重绘
                el_context.window_event = Some(event);
                if container.input(&mut el_context) {
                    wgcontext.present(&glob_pipeline, &container)
                }
            }
            Event::RedrawRequested(window_id)
            if window_id == el_context.window.id() => {
                wgcontext.present(&glob_pipeline, &container)
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
