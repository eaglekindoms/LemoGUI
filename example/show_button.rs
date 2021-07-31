use simple_logger::SimpleLogger;
use winit::event::VirtualKeyCode::Key1;

use LemoGUI::device::display_window::{DisplayWindow, WGContext};
use LemoGUI::device::container::Container;
use LemoGUI::graphic::base::color::*;
use LemoGUI::graphic::base::shape::{Point, Rectangle};
use LemoGUI::graphic::style::*;
use LemoGUI::widget::button::Button;
use LemoGUI::widget::frame::Frame;
use LemoGUI::widget::listener::State;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    log::info!("build window");
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("hello")
        .with_inner_size(winit::dpi::LogicalSize::new(428.0, 433.0));

    DisplayWindow::start_window::<Frame>(builder, &frame)
}

fn frame(wgcontext: WGContext) -> Frame
{
    // 自定义设置
    let rect = Rectangle::new(100.0, 100.0, 170, 40);
    let style = Style::default()
        .no_border()
        // .border(RGBA([0.1, 0.5, 0.2, 1.0]))
        .hover_color(RGBA(0.5, 0.0, 0.5, 0.5))
        .back_color(RGBA(0.4, 0.8, 0.8, 1.0))
        .font_color(RGBA(0.0, 0.0, 0.0, 1.0))
        .round();
    let button = Button::new_with_style(rect, style, "button1");
    let mut frame = Frame::new(wgcontext);
    frame.add_comp(button);
    frame
        .add_comp(
            Button::new(
                Point { x: 100.0, y: 300.0 },
                "按钮2")
                .update_state(Some(State::new(Some(Key1)))));
    frame
}
