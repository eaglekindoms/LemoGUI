use crate::graphic::base::*;
use crate::graphic::style::{Bordering, Rounding, Style};

/// 圆形顶点数据布局结构体
/// 顶点顺序为左下开始逆时针排序
#[repr(C)]
#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CircleVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub radius: f32,
    pub edge: u32,
}

impl CircleVertex {
    pub fn new(point: &Circle, edge: u32, color: RGBA) -> Self {
        log::info!("create the PolygonVertex obj");
        Self {
            position: [point.position.x, point.position.y],
            color: color.to_vec(),
            radius: point.radius,
            edge,
        }
    }
}

/// 矩形顶点数据布局结构体
#[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RectVertex {
    pub size: [f32; 2],
    pub position: [f32; 2],
    pub border_color: [f32; 4],
    pub rect_color: [f32; 4],
    pub is_round_or_border: [u32; 2],
}

impl RectVertex {
    pub fn new(rect: &Rectangle, style: Style) -> RectVertex {
        let mut border_color = [0.0, 0.0, 0.0, 0.0];
        let rect_color = style.get_display_color().to_vec();
        let is_round;
        let is_border;
        match style.get_round() {
            Rounding::Round => is_round = 1,
            Rounding::NoRound => is_round = 0,
        }
        match style.get_border() {
            Bordering::Border(color) => {
                is_border = 1;
                border_color = color.to_vec()
            }
            Bordering::NoBorder => is_border = 0,
        }
        RectVertex {
            size: [rect.width as f32, rect.height as f32],
            position: [rect.position.x, rect.position.y],
            border_color,
            rect_color,
            is_round_or_border: [is_round, is_border],
        }
    }
}

/// 多边形顶点数据布局结构体
#[repr(C)]
#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl PointVertex {
    pub fn new(x: f32, y: f32, color: RGBA) -> Self {
        log::info!("create the PointVertex obj");
        Self {
            position: [x, y],
            color: color.to_vec(),
        }
    }
}
