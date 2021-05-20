#[derive(Copy, Clone, Debug)]
pub enum Color {
    /// Red, Green, Blue, Alpha - All values' scales represented between 0.0 and 1.0.
    Rgba(f32, f32, f32, f32),
    /// Red, Green, Blue, Alpha - All values' scales represented between 0  and 255.
    Rgbau(u32, u32, u32, u32),
    /// Hue, Saturation, Lightness, Alpha - all values scales represented between 0.0 and 1.0.
    Hsla(f32, f32, f32, f32),
}

/// 颜色结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RGBA(pub [f32; 4]);

impl RGBA {
    pub fn to_u8(&self) -> (u8, u8, u8, u8) {
        let r = (*self.0.iter().next().unwrap() * 255.0) as u8;
        let g = (*self.0.iter().next().unwrap() * 255.0) as u8;
        let b = (*self.0.iter().next().unwrap() * 255.0) as u8;
        let a = (*self.0.iter().next().unwrap() * 255.0) as u8;
        (r, g, b, a)
    }
}

