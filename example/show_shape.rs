use std::path::Path;

use image::GenericImageView;
use simple_logger::SimpleLogger;
use winit::window::Icon;

use LemoGUI::device::container::Container;
use LemoGUI::device::display_window::*;
use LemoGUI::device::wgpu_context::WGContext;
use LemoGUI::graphic::base::color::*;
use LemoGUI::graphic::base::shape::*;
use LemoGUI::graphic::style::*;
use LemoGUI::widget::drawing_board::ShapeBoard;
use LemoGUI::widget::frame::Frame;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    log::info!("build window");
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/res/icon.png");

    let icon = load_icon(Path::new(path));

    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("hello")
        .with_inner_size(winit::dpi::LogicalSize::new(428.0, 633.0))
        .with_window_icon(Some(icon));

    start(builder, &frame)
}

fn frame(wgcontext: WGContext) -> Frame<()>
{
    let mut frame = Frame::new(wgcontext);
    frame.add_comp(shapes());
    frame
}

fn shapes() -> ShapeBoard {
    let mut shapes: Vec<Box<dyn ShapeGraph>> = Vec::with_capacity(10);
    let rect = Rectangle::new(21.0, 31.0, 221, 111);
    let rect2 = Rectangle::new(21.0, 181.0, 221, 111);
    let circle = Circle::new(401., 160.2, 110.2);
    let triangle = RegularPolygon::new(
        Circle::new(331., 560.2, 100.2), 3);

    let polygon = RegularPolygon::new(
        Circle::new(131., 510.2, 110.2), 6);

    let rects = RegularPolygon::new(
        Circle::new(631., 510.2, 110.2), 4);
    let points = Polygon::new(vec![
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
    shapes.push(Box::new(points));
    shapes.push(Box::new(rects));
    let style = Style::default().back_color(LIGHT_BLUE);

    ShapeBoard {
        shape_arr: shapes,
        style,
    }
}

fn load_icon(path: &Path) -> Icon {
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

