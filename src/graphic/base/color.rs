/// 颜色结构体
#[repr(C)]
#[derive(Copy, Default, Clone, Debug, PartialEq)]
pub struct RGBA(pub f32, pub f32, pub f32, pub f32);

pub const ALPHA: RGBA = RGBA(0.0, 0.0, 0.0, 0.0);
pub const BLACK: RGBA = RGBA(0.0, 0.0, 0.0, 1.0);
pub const WHITE: RGBA = RGBA(1.0, 1.0, 1.0, 1.0);
pub const LIGHT_WHITE: RGBA = RGBA(0.8, 0.8, 0.8, 1.0);
pub const LIGHT_BLUE: RGBA = RGBA(0.0, 0.75, 1.0, 0.5);
pub const RED: RGBA = RGBA(0.82, 0.12, 0.12, 1.0);
pub const GREEN: RGBA = RGBA(0.12, 0.55, 0.18, 1.0);
pub const BLUE: RGBA = RGBA(0.12, 0.32, 0.82, 1.0);
pub const ORANGE: RGBA = RGBA(0.9, 0.5, 0.08, 1.0);
pub const PURPLE: RGBA = RGBA(0.55, 0.22, 0.72, 1.0);
pub const GRAY: RGBA = RGBA(0.42, 0.42, 0.42, 1.0);

/// 默认窗口帧背景色
pub const BACKGROUND_COLOR: RGBA = RGBA(0.9, 0.9, 0.9, 1.0);

impl RGBA {
    /// 转化为u8元组
    pub fn to_u8(&self) -> [u8; 4] {
        let r = (self.0 * 255.0) as u8;
        let g = (self.1 * 255.0) as u8;
        let b = (self.2 * 255.0) as u8;
        let a = (self.3 * 255.0) as u8;
        [r, g, b, a]
    }
    /// 转化为浮点数组
    pub fn to_vec(&self) -> [f32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}
