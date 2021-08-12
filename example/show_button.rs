use simple_logger::SimpleLogger;
use winit::event::VirtualKeyCode::Key1;

use LemoGUI::device::container::Container;
use LemoGUI::device::display_window::*;
use LemoGUI::device::wgpu_context::WGContext;
use LemoGUI::graphic::base::color::*;
use LemoGUI::graphic::base::shape::{Point, Rectangle};
use LemoGUI::graphic::style::*;
use LemoGUI::widget::button::Button;
use LemoGUI::widget::frame::Frame;
use LemoGUI::widget::message::Message;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Ms {
    Cu
}

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    log::info!("build window");
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("hello")
        .with_inner_size(winit::dpi::LogicalSize::new(428.0, 433.0));

    start(builder, &frame)
}

fn frame(wgcontext: WGContext) -> Frame<Ms>
{
    // 自定义设置
    let rect = Rectangle::new(100.0, 100.0, 170, 40);
    let style = Style::default()
        .no_border()
        .hover_color(RGBA(0.0, 0.75, 1.0, 0.5))
        .back_color(RGBA(1.0, 0.5, 0.5, 1.0))
        .font_color(RGBA(0.0, 0.0, 0.0, 1.0))
        .round();
    let button
        = Button::new_with_style(rect, style, "button1")
        .set_list(Ms::Cu);
    let mut frame = Frame::new(wgcontext);
    frame.add_comp(button);
    frame
        .add_comp(
            Button::new(
                Point { x: 100.0, y: 300.0 },
                "按钮2")
                .message(Message::key(Key1))
                .set_custom(Ms::Cu));
    frame
}

fn call() {
    log::info!("hello");
}