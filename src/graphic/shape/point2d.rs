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

/// 二维顶点结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// 2d图形缓存顶点结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointVertex {
    pub position: Point,
    pub color: RGBA,
}

impl PointVertex {
    pub fn new(x: f32, y: f32, color: RGBA) -> Self {
        log::info!("create the BufferPoint obj");
        Self {
            position: Point { x, y },
            color,
        }
    }
}