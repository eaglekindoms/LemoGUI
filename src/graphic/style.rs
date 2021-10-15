use crate::graphic::base::*;

/// 边框枚举
#[derive(Copy, Clone, Debug)]
pub enum Bordering {
    /// 边框颜色
    Border(RGBA),
    /// 无边框
    NoBorder,
}

/// 圆角枚举
#[derive(Copy, Clone, Debug)]
pub enum Rounding {
    /// 圆角宽度和颜色
    Round,
    /// 无圆角
    NoRound,
}

/// 字体样式枚举
#[derive(Copy, Clone, Debug)]
pub enum FontStyle {
    // 无字体
    NoFont,
    // 字体颜色
    Font(RGBA),
}

/// 形状样式
#[derive(Copy, Clone, Debug)]
pub struct ShapeStyle {
    /// 是否有边界
    border: Bordering,
    /// 是否圆角
    round: Rounding,
    /// 背景色
    back_color: RGBA,
    /// 悬浮色
    hover_color: RGBA,
    // 默认背景显示颜色
    display_color: RGBA,
}

/// 样式结构体
/// 作用：设置图形样式
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// 文字样式
    font_style: FontStyle,
    /// 形状样式
    shape_style: ShapeStyle,
}

impl Style {
    pub fn default() -> Style {
        Style {
            font_style: FontStyle::NoFont,
            shape_style: ShapeStyle {
                border: Bordering::Border(BLACK),
                round: Rounding::NoRound,
                back_color: LIGHT_WHITE,
                hover_color: LIGHT_BLUE,
                display_color: LIGHT_WHITE,
            },
        }
    }
    pub fn border(&mut self, color: RGBA) -> Self {
        self.shape_style.border = Bordering::Border(color);
        *self
    }

    pub fn no_border(&mut self) -> Self {
        self.shape_style.border = Bordering::NoBorder;
        *self
    }

    pub fn round(&mut self) -> Self {
        self.shape_style.round = Rounding::Round;
        *self
    }
    pub fn no_round(&mut self) -> Self {
        self.shape_style.round = Rounding::NoRound;
        *self
    }

    pub fn font_color(&mut self, color: RGBA) -> Self {
        self.font_style = FontStyle::Font(color);
        *self
    }

    pub fn back_color(&mut self, color: RGBA) -> Self {
        self.shape_style.back_color = color;
        self.shape_style.display_color = color;
        *self
    }

    pub fn hover_color(&mut self, color: RGBA) -> Self {
        self.shape_style.hover_color = color;
        *self
    }

    pub fn display_color(&mut self, color: RGBA) -> Self {
        self.shape_style.display_color = color;
        *self
    }

    pub fn get_border(&self) -> &Bordering {
        &self.shape_style.border
    }

    pub fn get_round(&self) -> &Rounding {
        &self.shape_style.round
    }

    pub fn get_back_color(&self) -> RGBA {
        self.shape_style.back_color
    }

    pub fn get_font_color(&self) -> RGBA {
        match self.font_style {
            FontStyle::NoFont => DEFAULT_FONT_COLOR,
            FontStyle::Font(color) => color
        }
    }

    pub fn get_hover_color(&self) -> RGBA {
        self.shape_style.hover_color
    }

    pub fn get_display_color(&self) -> RGBA {
        self.shape_style.display_color
    }
}

impl Default for Bordering {
    fn default() -> Self {
        Bordering::NoBorder
    }
}
//
// impl Bordering {
//     pub fn get_color(&self) -> RGBA {
//         match self {
//             Bordering::Border(color) => color.clone(),
//             Bordering::NoBorder => RGBA([0.0, 0.0, 0.0, 0.0])
//         }
//     }
// }

impl Default for Rounding {
    fn default() -> Self {
        Rounding::NoRound
    }
}