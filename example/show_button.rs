use std::fmt::Debug;
use std::path::Path;

use simple_logger::SimpleLogger;
use winit::event::ElementState;
use winit::event::VirtualKeyCode::Key1;

use LemoGUI::device::container::Container;
use LemoGUI::device::display_window::*;
use LemoGUI::device::event_context::ELContext;
use LemoGUI::graphic::base::color::*;
use LemoGUI::graphic::base::shape::{Point, Rectangle};
use LemoGUI::graphic::render_middle::render_function::RenderUtil;
use LemoGUI::graphic::style::*;
use LemoGUI::widget::button::Button;
use LemoGUI::widget::component::ComponentModel;
use LemoGUI::widget::frame::Frame;
use LemoGUI::widget::text_input::TextInput;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Ms {
    Add,
    Sub,
}

struct Counter<M: Copy + PartialEq> {
    value: i32,
    b1: Button<M>,
    b2: Button<M>,
}

impl<M: Copy + PartialEq> ComponentModel<M> for Counter<M> {
    fn draw(&self, render_utils: &mut RenderUtil) {
        let v = TextInput::<M>::new(Point::new(120., 20.), self.value.to_string());
        v.draw(render_utils);
        self.b1.draw(render_utils);
        self.b2.draw(render_utils);
    }

    fn action_listener(&mut self,
                       action_state: ElementState,
                       el_context: &ELContext<'_, M>) -> bool {
        self.b1.action_listener(action_state, el_context) ||
            self.b2.action_listener(action_state, el_context)
    }

    fn message_listener(&mut self, broadcast: &M) -> bool {
        if self.b1.match_message(broadcast) {
            self.value += 1;
        }
        if self.b2.match_message(broadcast) {
            self.value -= 1;
        }
        true
    }
}

fn build_counter() -> Counter<Ms> {
    // 自定义设置
    let rect = Rectangle::new(100.0, 100.0, 170, 40);
    let style = Style::default()
        .no_border()
        .hover_color(RGBA(0.0, 0.75, 1.0, 0.5))
        .back_color(RGBA(1.0, 0.5, 0.5, 1.0))
        .font_color(RGBA(0.0, 0.0, 0.0, 1.0), 45.)
        .round();
    let b1 = Button::new_with_style(rect, style, "add button 加")
        .action(Ms::Add);
    let b2 = Button::new(Point::new(100.0, 200.0), "sub button 减")
        .action(Ms::Sub);

    Counter {
        value: 0,
        b1,
        b2,
    }
}

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    log::info!("build window");
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/res/icon.png");

    let icon = load_icon(Path::new(path));
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("Counter")
        .with_inner_size(winit::dpi::LogicalSize::new(428.0, 433.0))
        .with_window_icon(Some(icon));
    let display_window = DisplayWindow::new(builder);
    let mut frame = display_window.request_container::<Frame<Ms>>();
    frame.add_comp(build_counter());
    display_window.start(frame);
}