use crate::backend::wgpu_impl::VertexBuffer;
use crate::device::GPUContext;
use crate::graphic::style::Style;

/// 点结构体
#[repr(C)]
#[derive(Copy, Default, Clone, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}


/// 矩形结构体
#[derive(Debug, Default, Copy, Clone)]
pub struct Rectangle {
    /// 左上顶点坐标
    pub position: Point<f32>,
    /// 宽度
    pub width: u32,
    /// 长度
    pub height: u32,
}

/// 圆形结构体
#[derive(Debug, Default, Copy, Clone)]
pub struct Circle {
    /// 圆心坐标
    pub position: Point<f32>,
    /// 半径
    pub radius: f32,
}

/// 正多边形结构体
///
/// 根据给定圆和边数作的圆内切正多边形
#[derive(Debug)]
pub struct RegularPolygon {
    /// 外接圆
    pub point: Circle,
    /// 边数
    pub edge: u32,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point {
            x,
            y,
        }
    }
}

/// 多边形结构体
///
/// 根据给定顶点坐标绘制任意多边形
/// 顶点顺序为多边形的逆时针排列
#[derive(Debug)]
pub struct Polygon {
    pub points: Vec<Point<f32>>,
}

impl Polygon {
    pub fn new(points: Vec<Point<f32>>) -> Polygon {
        Polygon {
            points
        }
    }
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: u32, h: u32) -> Rectangle {
        Rectangle {
            position: Point { x, y },
            width: w,
            height: h,
        }
    }


    /// 将矩形映射到给定宽高的区域中，坐标范围变为-1.0~1.0
    pub fn get_coord(&self, w_width: u32, w_height: u32) -> (f32, f32, f32, f32) {
        (2.0 * self.position.x as f32 / w_width as f32 - 1.0,
         1.0 - 2.0 * self.position.y as f32 / w_height as f32,
         2.0 * self.width as f32 / w_width as f32,
         2.0 * self.height as f32 / w_height as f32)
    }

    /// 判断点是否在矩形内
    pub fn contain_coord(&self, position: Point<f32>) -> bool {
        let rel_x = position.x - self.position.x;
        let rel_y = position.y - self.position.y;
        (rel_x < (self.width as f32)) &&
            (rel_y < (self.height as f32)) &&
            (rel_x > 0.) &&
            (rel_y > 0.)
    }
}

impl Circle {
    pub fn new(x: f32, y: f32, r: f32) -> Circle {
        Circle {
            position: Point::new(x, y),
            radius: r,
        }
    }
}

impl RegularPolygon {
    pub fn new(point: Circle, edge: u32) -> RegularPolygon {
        RegularPolygon {
            point,
            edge,
        }
    }
}

/// 图形缓冲转换接口
pub trait ShapeGraph {
    /// 转换为顶点缓冲数据
    fn to_buffer(&self, gpu_context: &GPUContext, style: Style) -> VertexBuffer;
}
