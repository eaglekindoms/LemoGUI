use simple_logger::SimpleLogger;

use LemoGUI::device::container::Container;
use LemoGUI::device::display_window::*;
use LemoGUI::graphic::base::color::*;
use LemoGUI::graphic::base::shape::*;
use LemoGUI::graphic::style::*;
use LemoGUI::widget::drawing_board::ShapeBoard;
use LemoGUI::widget::frame::Frame;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    log::info!("build window");
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("hello")
        .with_inner_size(winit::dpi::LogicalSize::new(428.0, 633.0));

    DisplayWindow::start_window::<Frame>(builder, &frame)
}

fn frame(wgcontext: WGContext) -> Frame
{
    let mut frame = Frame::new(wgcontext);
    frame.add_comp(shapes());
    frame
}

fn shapes() -> ShapeBoard {
    let mut shapes: Vec<Box<dyn ShapeBuffer>> = Vec::with_capacity(10);
    let rect = Rectangle::new(21.0, 31.0, 221, 111);
    let rect2 = Rectangle::new(21.0, 181.0, 221, 111);
    let circle = Circle::new(401., 160.2, 110.2);
    let triangle = Polygon::new(
        Circle::new(331., 560.2, 100.2), 3);

    let polygon = Polygon::new(
        Circle::new(131., 510.2, 110.2), 6);

    let rects = Polygon::new(
        Circle::new(631., 510.2, 110.2), 4);

    shapes.push(Box::new(rect));
    shapes.push(Box::new(rect2));
    shapes.push(Box::new(circle));
    shapes.push(Box::new(triangle));
    shapes.push(Box::new(polygon));
    shapes.push(Box::new(rects));
    let style = Style::default().back_color(LIGHT_BLUE);

    ShapeBoard {
        shape_arr: shapes,
        style,
    }
}


