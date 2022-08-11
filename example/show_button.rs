use std::fmt::Debug;

use simple_logger::SimpleLogger;

use LemoGUI::graphic::base::*;
use LemoGUI::graphic::style::*;
use LemoGUI::instance::*;
use LemoGUI::widget::*;

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
    Counter::run();
}

#[derive(Debug, Clone, PartialEq)]
enum Ms {
    Add,
    Sub,
    Text(String),
}

struct Counter {
    value: i32,
    text: String,
}

impl Instance for Counter {
    type M = Ms;

    fn new() -> Self {
        Counter {
            value: 0,
            text: "文本测试".to_string(),
        }
    }

    fn layout(&self) -> Panel<Ms> {
        // 自定义设置
        let rect = Rectangle::new(100.0, 100.0, 80, 40);
        let style = Style::default()
            .border(RGBA(0.2, 0.2, 0.2, 0.5))
            .hover_color(RGBA(0.0, 0.75, 1.0, 0.5))
            .back_color(RGBA(1.0, 0.5, 0.5, 1.0))
            .font_color(RGBA(0.1, 0.3, 0.8, 1.0))
            .round();
        let b1 = Button::new_with_style(rect, style, "数字 +").action(Ms::Add);
        Panel::new()
            .push(Button::new(Point::new(300.0, 100.0), "数字 -").action(Ms::Sub))
            .push(TextInput::new(
                Point::new(200.0, 300.0),
                self.text.as_str(),
                Ms::Text,
            ))
            .push(b1)
            .push(Button::new(Point::new(230., 100.), self.value.to_string()))
    }

    fn update(&mut self, broadcast: &Ms) {
        match broadcast {
            Ms::Add => self.value += 1,
            Ms::Sub => self.value -= 1,
            Ms::Text(str) => self.text = str.to_string(),
        }
    }

    fn setting() -> Setting {
        log::info!("build window");

        let mut setting = Setting::default();
        setting.size = Point::new(428., 433.);
        setting.icon_path = Some(concat!(env!("CARGO_MANIFEST_DIR"), "/res/icon.png").into());
        setting
    }
}
