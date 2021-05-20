use log::{debug, error, info, Level, log_enabled};
use simple_logger::SimpleLogger;

use container::GlobalState;
use LemoGUI::device::display_window::DisplayWindow;
use LemoGUI::device::listener::Listener;
use LemoGUI::device::painter::Painter;
use LemoGUI::graphic::base::point2d::Point;
use LemoGUI::graphic::base::rectangle::Rectangle;
use LemoGUI::widget::button::Button;

pub mod container;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    info!("hh");
    run::<GlobalState>("hello");
}

fn run<E>(title: &str)
    where E: Painter + 'static
{
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title(title)
        .with_inner_size(winit::dpi::LogicalSize::new(428.0, 433.0));

    use futures::executor::block_on;
    let display_device = block_on(DisplayWindow::init::<E>(builder));
    // from window's variable to create the painter for render the shapes;
    log::info!("Initializing the example...");
    // 自定义设置
    let rect = Rectangle::new(100.0, 100.0, 400, 40);
    let mut button = Button::new(rect, "button1");
    log::info!("{:#?}", &button.index);
    let mut container = E::new(display_device.wgcontext);
    container.add_comp(&mut button);
    container.add_comp(&mut Button::default(Point { x: 100.0, y: 300.0 }, "hello"));
    log::info!("{:#?}", &button.index);
    DisplayWindow::start::<E>(display_device.event_loop, container);
    log::info!("{:#?}", &button.index);
}
