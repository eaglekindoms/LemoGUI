use crate::graphic::base::color::{BLACK, LIGHT_BLUE, LIGHT_WHITE, RGBA};

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

/// 样式结构体
/// 作用：设置图形样式
#[derive(Debug, Default, Clone, Copy)]
pub struct Style {
    /// 是否有边界
    border: Bordering,
    /// 是否圆角
    round: Rounding,
    /// 文字颜色
    font_color: RGBA,
    /// 背景色
    back_color: RGBA,
    /// 悬浮色
    hover_color: RGBA,
    // 默认背景显示颜色
    display_color: RGBA,
}

impl Style {
    pub fn default() -> Style {
        Style {
            border: Bordering::Border(BLACK),
            round: Rounding::NoRound,
            font_color: BLACK,
            back_color: LIGHT_WHITE,
            hover_color: LIGHT_BLUE,
            display_color: LIGHT_WHITE,
        }
    }
    pub fn border(&mut self, color: RGBA) -> Self {
        self.border = Bordering::Border(color);
        *self
    }

    pub fn no_border(&mut self) -> Self {
        self.border = Bordering::NoBorder;
        *self
    }

    pub fn round(&mut self) -> Self {
        self.round = Rounding::Round;
        *self
    }
    pub fn no_round(&mut self) -> Self {
        self.round = Rounding::NoRound;
        *self
    }

    pub fn font_color(&mut self, color: RGBA) -> Self {
        self.font_color = color;
        *self
    }

    pub fn back_color(&mut self, color: RGBA) -> Self {
        self.back_color = color;
        self.display_color = color;
        *self
    }

    pub fn hover_color(&mut self, color: RGBA) -> Self {
        self.hover_color = color;
        *self
    }

    pub fn display_color(&mut self, color: RGBA) -> Self {
        self.display_color = color;
        *self
    }

    pub fn get_border(&self) -> &Bordering {
        &self.border
    }

    pub fn get_round(&self) -> &Rounding {
        &self.round
    }

    pub fn get_back_color(&self) -> RGBA {
        self.back_color
    }

    pub fn get_font_color(&self) -> RGBA {
        self.font_color
    }

    pub fn get_hover_color(&self) -> RGBA {
        self.hover_color
    }

    pub fn get_display_color(&self) -> RGBA {
        self.display_color
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