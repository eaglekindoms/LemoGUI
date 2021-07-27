use simple_logger::SimpleLogger;

use LemoGUI::device::display_window::{DisplayWindow, WGContext};
use LemoGUI::device::painter::Painter;
use LemoGUI::graphic::base::color::*;
use LemoGUI::graphic::base::shape::*;
use LemoGUI::graphic::render_middle::pipeline_state::PipelineState;
use LemoGUI::graphic::render_middle::render_function::RenderUtil;
use LemoGUI::graphic::style::*;
use LemoGUI::widget::component::ComponentModel;
use LemoGUI::widget::container::Container;
use LemoGUI::widget::listener::Listener;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    log::info!("build window");
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("hello")
        .with_inner_size(winit::dpi::LogicalSize::new(428.0, 433.0));

    use futures::executor::block_on;
    let display_device = block_on(DisplayWindow::init::<Container>(builder));
    // from window's variable to create the painter for render the shapes;
    log::info!("Initializing the example...");
    DisplayWindow::start::<Container>(display_device.window, display_device.event_loop,
                                      build_container(display_device.wgcontext));
}

fn build_container(wgcontext: WGContext) -> Container
{
    let mut container = Container::new(wgcontext);
    container.add_comp(shapes());
    container
}

fn shapes() -> Shape {
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

    Shape {
        shape_arr: shapes,
        style,
    }
}

struct Shape {
    shape_arr: Vec<Box<dyn ShapeBuffer>>,
    style: Style,
}

impl ComponentModel for Shape {
    fn set_glob_pipeline(&self, wgcontext: &WGContext, glob_pipeline: &mut PipelineState) {
        glob_pipeline.set_pipeline(&wgcontext.device, ShapeType::ROUND);
        glob_pipeline.set_pipeline(&wgcontext.device, ShapeType::CIRCLE);
        glob_pipeline.set_pipeline(&wgcontext.device, ShapeType::POLYGON);
    }

    fn draw(&mut self, wgcontext: &WGContext, render_utils: &mut RenderUtil, glob_pipeline: &PipelineState) {
        let mut style = self.style;
        for shape in &self.shape_arr {
            shape.to_buffer(wgcontext, &style).render(render_utils, &glob_pipeline, shape.get_type());
            style = Style::default().back_color(LIGHT_BLUE).round();
        }
    }
}

impl Listener for Shape {}
