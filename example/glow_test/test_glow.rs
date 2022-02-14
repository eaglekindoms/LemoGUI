use glow::HasContext;
use raw_window_handle::HasRawWindowHandle;
use simple_logger::SimpleLogger;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use lemo_gui::graphic::base::{BACKGROUND_COLOR, WHITE};

fn main() {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new().with_title("gl test");
    use lemo_gui::backend::glow_impl::*;
    let (context, window) = GLGPUContext::new(wb, &event_loop);
    let render_util = GRenderUtil::new();
    event_loop.run(move |event, _, control_flow| {
        println!("{:?}", event);

        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    ..
                } => {
                    window.request_redraw();
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                println!("\nredrawing!\n");
                context.clear_frame(BACKGROUND_COLOR);
            }
            _ => (),
        }
    });
}
