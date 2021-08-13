use winit::event::{KeyboardInput, VirtualKeyCode};

/// a send message
/// m broadcast message
/// b listen message
/// a <---> m <---> b
/// 1. define custom_event
/// 2. create event_loop
/// 3. async send custom_event by event_loop_proxy
/// 4. run event_loop match events
///
/// 思路重置
/// 事件是事件，消息是消息，两者需要区分开
/// 1.鼠标点击事件，
/// api: set_action(message) :指监听到鼠标点击事件后，广播该消息
/// 所以需要有一个广播来发送和接收消息，（之前是使用了事件广播器来实现，实现简单，但是无法自定义监听）
///
/// 需要提供的组件api
/// 1. 指定事件发生时需广播的消息 set_action(message), set_keyboard(key, message)
///
/// 2.全局广播器：channel<message>() : sender, receiver
/// sender 不要暴露出来，要提供给事件监听器，在监听到组件关注事件时，调用该sender发送事件
/// receiver 提供给组件创建所需的上下文中，根据具体逻辑接收消息实现组件间的交互
///
/// 结构体设置：
/// 事件类型枚举
#[derive(Debug)]
enum EventType {
    mouse,
    KeyBoard(VirtualKeyCode),
}

/// 组件状态结构体，记录绑定的事件、及与事件联动的消息
#[derive(Debug)]
struct State<M> {
    event: EventType,
    message: Option<M>,
}

fn main() {
    use simple_logger::SimpleLogger;
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    #[derive(Debug, Clone, Copy)]
    enum CustomEvent {
        Timer,
    }

    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::<CustomEvent>::with_user_event();

    let _window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    // `EventLoopProxy` allows you to dispatch custom events to the main Winit event
    // loop from any thread.
    let event_loop_proxy = event_loop.create_proxy();

    std::thread::spawn(move || {
        // Wake up the `event_loop` once every second and dispatch a custom event
        // from a different thread.
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            event_loop_proxy.send_event(CustomEvent::Timer).ok();
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(event) => println!("user event: {:?}", event),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
