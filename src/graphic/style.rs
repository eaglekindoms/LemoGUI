use crate::graphic::base::color::Color;

///  边框
#[derive(Copy, Clone, Debug)]
pub enum Bordering {
    /// 边框宽度和颜色
    Border(f64, Color),
    /// 无边框
    NoBorder,
}

#[derive(Copy, Clone, Debug)]
pub enum Label {
    /// 面板颜色
    Label(Color),
    /// 无面板
    NoLabel,
}

#[derive(Debug)]
pub struct Style {
    border: Bordering,
    label: Label,
}

impl Style {
    pub fn default() -> Style {
        Style {
            border: Bordering::Border(1.0, Color::Rgbau(0, 0, 0, 255)),
            label: Label::Label(Color::Rgbau(255, 255, 255, 255)),
        }
    }
    pub fn set_border(&mut self, border: Bordering) -> Self {
        Style {
            border,
            label: Label::Label(Color::Rgbau(255, 255, 255, 255)),
        }
    }
}

impl Bordering {
    /// Set the width of the widget's border.
    pub fn border(self, width: f64, color: Color) -> Self {
        self::Bordering::Border(width, color)
    }
}
