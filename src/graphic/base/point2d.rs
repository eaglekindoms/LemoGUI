use crate::graphic::base::color::RGBA;

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