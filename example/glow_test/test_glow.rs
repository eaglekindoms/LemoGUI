use glow::HasContext;
use raw_window_handle::HasRawWindowHandle;
use simple_logger::SimpleLogger;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use lemo_gui::backend::wgpu_impl::RenderUtil;
use lemo_gui::graphic::base::{Rectangle, BACKGROUND_COLOR, WHITE, ShapeGraph};
use lemo_gui::graphic::render_api::PaintBrush;
use lemo_gui::graphic::style::Style;

fn main() {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();
    let rect = Rectangle::new(21.0, 31.0, 221, 111);

    let wb = WindowBuilder::new().with_title("gl test");
    use lemo_gui::backend::glow_impl::*;
    let (mut context, window) = GLGPUContext::new(wb, &event_loop);
    event_loop.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        let mut render_util = GRenderUtil::new(&mut context);

        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    ..
                } => {
                    println!(" redrawing! ");
                    let rec:Box<dyn ShapeGraph>=Box::new(rect);
                    render_util.draw_shape(&rec, Style::default());
                    window.request_redraw();
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                println!(" redrawing! ");
                render_util.clear_frame(BACKGROUND_COLOR);
            }
            _ => (),
        }
    });
}
