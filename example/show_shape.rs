use simple_logger::SimpleLogger;

use LemoGUI::device::display_window::*;
use LemoGUI::device::container::Container;
use LemoGUI::graphic::base::color::*;
use LemoGUI::graphic::base::shape::*;
use LemoGUI::graphic::style::*;
use LemoGUI::widget::frame::Frame;
use LemoGUI::widget::drawing_board::ShapeBoard;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    log::info!("build window");
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("hello")
        .with_inner_size(winit::dpi::LogicalSize::new(428.0, 433.0));

    DisplayWindow::start_window::<Frame>(builder, &build_container)
}

fn build_container(wgcontext: WGContext) -> Frame
{
    let mut container = Frame::new(wgcontext);
    container.add_comp(shapes());
    container
}

fn shapes() -> ShapeBoard {
    let mut shapes: Vec<Box<dyn ShapeBuffer>> = Vec::with_capacity(10);
    let rect = Rectangle::new(21.0, 31.0, 221, 111);
    let rect2 = Rectangle::new(21.0, 181.0, 221, 111);
    let circle = Circle::new(0.5, 0.5, 0.2);
    let triangle = Polygon::new(vec![
        Point::new(-0.4, -0.1),
        Point::new(-0.90, -0.60),
        Point::new(-0.10, -0.70),
    ]);

    let polygon = Polygon::new(vec![
        Point::new(0.2, -0.6),//0
        Point::new(0.4, -0.6),//1
        Point::new(0.5, -0.4),//2
        Point::new(0.4, -0.2),//3
        Point::new(0.2, -0.2),//4
        Point::new(0.1, -0.4),//5
    ]);

    shapes.push(Box::new(rect));
    shapes.push(Box::new(rect2));
    shapes.push(Box::new(circle));
    shapes.push(Box::new(triangle));
    shapes.push(Box::new(polygon));
    let style = Style::default().back_color(LIGHT_BLUE);

    ShapeBoard {
        shape_arr: shapes,
        style,
    }
}


