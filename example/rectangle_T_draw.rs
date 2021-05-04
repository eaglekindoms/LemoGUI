use log::{debug, error, info, Level, log_enabled};
use simple_logger::SimpleLogger;

use global_setting::GlobalState;
use LemoGUI::device::display_window::DisplayWindow;
use LemoGUI::device::painter::Painter;

pub mod global_setting;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    info!("hh");
    run::<GlobalState>("hello");
}

fn run<E: Painter>(title: &str) {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title(title)
        .with_inner_size(winit::dpi::LogicalSize::new(428.0, 128.0));

    use futures::executor::block_on;
    let display_device = block_on(DisplayWindow::init::<E>(builder, &event_loop));
    DisplayWindow::start::<E>(display_device, event_loop);
}
