/// 颜色结构体
#[repr(C)]
#[derive(Copy, Default, Clone, Debug)]
pub struct RGBA(pub f32, pub f32, pub f32, pub f32);

pub const ALPHA: RGBA = RGBA(0.0, 0.0, 0.0, 0.0);
pub const BLACK: RGBA = RGBA(0.0, 0.0, 0.0, 1.0);
pub const WHITE: RGBA = RGBA(1.0, 1.0, 1.0, 1.0);
pub const LIGHT_WHITE: RGBA = RGBA(0.8, 0.8, 0.8, 1.0);
pub const LIGHT_BLUE: RGBA = RGBA(0.0, 0.75, 1.0, 0.5);

/// 默认窗口帧背景色
pub const BACKGROUND_COLOR: RGBA = RGBA(0.9, 0.9, 0.9, 1.0);

impl RGBA {
    /// 转化为u8元组
    pub fn to_u8(&self) -> (u8, u8, u8, u8) {
        let r = (self.0 * 255.0) as u8;
        let g = (self.1 * 255.0) as u8;
        let b = (self.2 * 255.0) as u8;
        let a = (self.3 * 255.0) as u8;
        (r, g, b, a)
    }
    /// 转化为浮点数组
    pub fn to_vec(&self) -> [f32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

