use crate::graphic::base::{RGBA, BLACK};

/// 行对齐
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Align {
    pub fn as_u8(self) -> u8 {
        match self {
            Align::Left => 0,
            Align::Center => 1,
            Align::Right => 2,
        }
    }

    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Align::Center,
            2 => Align::Right,
            _ => Align::Left,
        }
    }
}

/// 富文本样式（颜色为着色器 tint，加粗/斜体在绘制时合成）
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub color: RGBA,
    pub size: f32,
    pub align: Align,
    pub font: u8,
}

/// ComboBox / 缓存共用的字号分档
pub const FONT_SIZE_BUCKETS: [u32; 11] = [12, 14, 16, 18, 20, 24, 28, 32, 36, 40, 48];

/// 将任意字号落到最近分档
pub fn quantize_size(size: f32) -> u32 {
    let s = size.max(1.0);
    let mut best = FONT_SIZE_BUCKETS[0];
    let mut best_d = (s - best as f32).abs();
    for &bucket in &FONT_SIZE_BUCKETS[1..] {
        let d = (s - bucket as f32).abs();
        if d < best_d {
            best = bucket;
            best_d = d;
        }
    }
    best
}

impl TextStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = quantize_size(size) as f32;
        self
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            bold: false,
            italic: false,
            underline: false,
            color: BLACK,
            size: 16.0,
            align: Align::Left,
            font: 0,
        }
    }
}
