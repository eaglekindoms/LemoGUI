use std::fmt::Debug;
use std::path::Path;

use simple_logger::SimpleLogger;
use winit::event::ElementState;
use winit::event::VirtualKeyCode::Key1;

use LemoGUI::device::*;
use LemoGUI::graphic::base::*;
use LemoGUI::graphic::style::*;
use LemoGUI::widget::*;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    Counter::run();
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Ms {
    Add,
    Sub,
}

struct Counter {
    value: i32,
}

impl Instance for Counter {
    type M = Ms;

    fn new() -> Self {
        Counter {
            value: 0,
        }
    }

    fn layout(&self) -> Panel<Ms> {
        // 自定义设置
        let rect = Rectangle::new(100.0, 100.0, 170, 40);
        let style = Style::default()
            .no_border()
            .hover_color(RGBA(0.0, 0.75, 1.0, 0.5))
            .back_color(RGBA(1.0, 0.5, 0.5, 1.0))
            .font_color(RGBA(0.1, 0.3, 0.8, 1.0), 45.)
            .round();
        let b1 = Button::new_with_style(rect, style, "add button 加")
            .action(Ms::Add);
        Panel::new()
            .push(Button::new(Point::new(100.0, 200.0), "sub button 减").action(Ms::Sub))
            .push(b1)
            .push(Button::new(Point::new(120., 20.), self.value.to_string()))
    }

    fn update(&mut self, broadcast: &Ms) {
        match broadcast {
            Ms::Add => { self.value += 1 }
            Ms::Sub => { self.value -= 1 }
        }
    }

    fn run() {
        log::info!("build window");
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/res/icon.png");

        let icon = load_icon(Path::new(path));
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_title("Counter")
            .with_inner_size(winit::dpi::LogicalSize::new(428.0, 433.0))
            .with_window_icon(Some(icon));
        let display_window = DisplayWindow::new(builder);
        let frame = display_window.request_container::<Frame<Ms>>();
        display_window.start(frame, Self::new());
    }
}
