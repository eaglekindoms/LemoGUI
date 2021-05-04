#[derive(Copy, Clone)]
pub enum Color {
    /// Red, Green, Blue, Alpha - All values' scales represented between 0.0 and 1.0.
    Rgba(f32, f32, f32, f32),
    /// Red, Green, Blue, Alpha - All values' scales represented between 0  and 255.
    Rgbau(u32, u32, u32, u32),
    /// Hue, Saturation, Lightness, Alpha - all values scales represented between 0.0 and 1.0.
    Hsla(f32, f32, f32, f32),
}

///  边框
#[derive(Copy, Clone)]
pub enum Bordering {
    /// 边框宽度和颜色
    Border(f64, Color),
    /// 无边框
    NoBorder,
}

#[derive(Copy, Clone)]
pub enum Label {
    /// 面板颜色
    Label(Color),
    /// 无面板
    NoLabel,
}

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
            border: Bordering::Border(1.0, Color::Rgbau(0, 0, 0, 255)),
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
