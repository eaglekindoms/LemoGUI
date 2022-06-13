use glow::HasContext;
use raw_window_handle::HasRawWindowHandle;
use simple_logger::SimpleLogger;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use lemo_gui::graphic::base::*;
use lemo_gui::graphic::render_api::PaintBrush;
use lemo_gui::graphic::style::Style;

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Trace)
        .init()
        .unwrap();
    let event_loop = EventLoop::new();
    let rect = Rectangle::new(21.0, 31.0, 221, 111);

    let wb = WindowBuilder::new().with_title("gl test");
    use lemo_gui::backend::glow_impl::*;
    let (mut context, window) = GLGPUContext::new_with_builder(wb, &event_loop);
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
                    println!("mous input redrawing! ");
                    render_util.clear_frame(LIGHT_WHITE);
                    let rec: Box<dyn ShapeGraph> = Box::new(rect);
                    render_util.draw_shape(&rec, Style::default());
                    let triangle: Box<dyn ShapeGraph> =
                        Box::new(RegularPolygon::new(Circle::new(331., 560.2, 100.2), 3));
                    render_util.draw_shape(&triangle, Style::default());
                    let rect2: Box<dyn ShapeGraph> = Box::new(Polygon::new(vec![
                        Point::new(0.2, -0.6), //0
                        Point::new(0.4, -0.6), //1
                        Point::new(0.5, -0.4), //2
                        Point::new(0.4, -0.2), //3
                        Point::new(0.2, -0.2), //4
                        Point::new(0.1, -0.4), //5
                    ]));
                    render_util.draw_shape(&rect2, Style::default());
                    // render_util.context.swap_buffers();
                    context.update_surface_configure(window.inner_size());
                    // window.request_redraw();
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                println!(" redrawing! ");
                // let rec:Box<dyn ShapeGraph>=Box::new(rect);
                // render_util.draw_shape(&rec, Style::default());
                render_util.clear_frame(LIGHT_WHITE);
            }
            _ => (),
        }
    });
}
