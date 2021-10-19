use std::path::Path;

use simple_logger::SimpleLogger;

use LemoGUI::device::*;
use LemoGUI::graphic::base::*;
use LemoGUI::graphic::style::*;
use LemoGUI::widget::{Instance, Panel};
use LemoGUI::widget::ShapeBoard;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    Board::run();
}


struct Board;

impl Instance for Board {
    type M = ();

    fn new() -> Self {
        Self
    }

    fn layout(&self) -> Panel<()> {
        // 自定义设置
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
        Panel::new().push(
            ShapeBoard {
                shape_arr: shapes,
                style,
            })
    }


    fn setting() -> winit::window::WindowBuilder {
        log::info!("build window");
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/res/icon.png");

        let icon = load_icon(Path::new(path));

        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_title("hello")
            .with_inner_size(winit::dpi::LogicalSize::new(428.0, 633.0))
            .with_window_icon(Some(icon));
        builder
    }
}