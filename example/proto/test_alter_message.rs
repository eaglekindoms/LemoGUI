/// a send message
/// m broadcast message
/// b listen message
/// a <---> m <---> b
/// 1. define custom_event
/// 2. create event_loop
/// 3. async send custom_event by event_loop_proxy
/// 4. run event_loop match events
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
