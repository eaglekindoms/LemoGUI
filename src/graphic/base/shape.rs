use crate::device::wgpu_context::WGContext;
use crate::graphic::base::color::*;
use crate::graphic::render_middle::circle_vertex::CircleVertex;
use crate::graphic::render_middle::rect_vertex::RectVertex;
use crate::graphic::render_middle::triangle_vertex::PointVertex;
use crate::graphic::render_middle::vertex_buffer::{RECT_INDEX, VertexBuffer};
use crate::graphic::style::Style;

/// 图形类型枚举
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ShapeType {
    TEXTURE = 0,
    ROUND = 1,
    BORDER = 2,
    POINT = 3,
    Circle = 4,
}

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
    pub position: Point<f32>,
    pub width: u32,
    pub height: u32,
    pub style: Option<Style>
}

/// 圆形结构体
#[derive(Debug, Default, Copy, Clone)]
pub struct Circle {
    pub position: Point<f32>,
    pub radius: f32,
}

/// 多边形结构体
#[derive(Debug)]
pub struct RegularPolygon {
    pub point: Circle,
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
            style: None,
        }
    }

    pub fn set_style(&mut self, style: Style) -> Self {
        self.style = Some(style);
        *self
    }

    #[deprecated]
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
        (rel_x < self.width as f32) &&
            (rel_y < self.height as f32) &&
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
    fn to_buffer(&self, wgcontext: &WGContext, color: RGBA) -> VertexBuffer;
    /// 获取图形类型
    fn get_type(&self) -> ShapeType;
}

impl ShapeGraph for Rectangle {
    fn to_buffer(&self, wgcontext: &WGContext, color: RGBA) -> VertexBuffer {
        let rect_vertex = RectVertex::new(&self, &wgcontext.sc_desc, color);
        let rect_vertex = VertexBuffer::create_vertex_buf::<RectVertex>
            (&wgcontext.device, vec![rect_vertex], RECT_INDEX);
        rect_vertex
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::ROUND
    }
}

impl ShapeGraph for Circle {
    fn to_buffer(&self, wgcontext: &WGContext, color: RGBA) -> VertexBuffer {
        let circle_vertex
            = CircleVertex::new(&self, 0, color);
        let cricle_buffer = VertexBuffer::create_vertex_buf::<CircleVertex>
            (&wgcontext.device, vec![circle_vertex], RECT_INDEX);
        cricle_buffer
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::Circle
    }
}

impl ShapeGraph for RegularPolygon {
    fn to_buffer(&self, wgcontext: &WGContext, color: RGBA) -> VertexBuffer {
        let circle_vertex
            = CircleVertex::new(&self.point, self.edge, color);
        let cricle_buffer = VertexBuffer::create_vertex_buf::<CircleVertex>
            (&wgcontext.device, vec![circle_vertex], RECT_INDEX);
        cricle_buffer
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::Circle
    }
}

impl ShapeGraph for Polygon {
    fn to_buffer(&self, wgcontext: &WGContext, color: RGBA) -> VertexBuffer {
        PointVertex::from_shape_to_vector(wgcontext, &self.points, color)
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::POINT
    }
}