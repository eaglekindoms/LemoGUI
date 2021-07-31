use crate::device::display_window::WGContext;
use crate::graphic::base::color::RGBA;
use crate::graphic::base::triangle_vertex::PointVertex;
use crate::graphic::base::poly_vertex::PolygonVertex;
use crate::graphic::base::rect_vertex::RectVertex;
use crate::graphic::render_middle::vertex_buffer::{RECT_INDEX, VertexBuffer};
use crate::graphic::style::Style;

/// 图形类型枚举
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ShapeType {
    TEXTURE = 0,
    ROUND = 1,
    BORDER = 2,
    POINT = 3,
    POLYGON = 4,
}

/// 点结构体
#[repr(C)]
#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}


/// 矩形结构体
#[derive(Debug, Default, Copy, Clone)]
pub struct Rectangle {
    pub position: Point,
    pub width: u32,
    pub height: u32,
}

/// 圆形结构体
#[derive(Debug, Default, Copy, Clone)]
pub struct Circle {
    pub position: Point,
    pub radius: f32,
}

/// 多边形结构体
#[derive(Debug)]
pub struct Polygon {
    pub point: Circle,
    pub edge: u32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {
            x,
            y,
        }
    }
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: u32, h: u32) -> Rectangle {
        log::info!("create the Rectangle obj");
        Rectangle {
            position: Point { x, y },
            width: w,
            height: h,
        }
    }

    #[deprecated]
    pub fn get_coord(&self, w_width: u32, w_height: u32) -> (f32, f32, f32, f32) {
        (2.0 * self.position.x as f32 / w_width as f32 - 1.0,
         1.0 - 2.0 * self.position.y as f32 / w_height as f32,
         2.0 * self.width as f32 / w_width as f32,
         2.0 * self.height as f32 / w_height as f32)
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

impl Polygon {
    pub fn new(point: Circle, edge: u32) -> Polygon {
        Polygon {
            point,
            edge,
        }
    }
}

/// 图形缓冲转换接口
pub trait ShapeBuffer {
    /// 转换为顶点缓冲数据
    fn to_buffer(&self, wgcontext: &WGContext, color: RGBA) -> VertexBuffer;
    /// 获取图形类型
    fn get_type(&self) -> ShapeType;
}

impl ShapeBuffer for Rectangle {
    fn to_buffer(&self, wgcontext: &WGContext, color: RGBA) -> VertexBuffer {
        RectVertex::from_shape_to_vector
            (&wgcontext.device, &wgcontext.sc_desc, &self, &Style::default())
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::ROUND
    }
}

impl ShapeBuffer for Circle {
    fn to_buffer(&self, wgcontext: &WGContext, color: RGBA) -> VertexBuffer {
        let circle_vertex
            = PolygonVertex::new(&self, 0, color);
        let cricle_buffer = VertexBuffer::create_vertex_buf::<PolygonVertex>
            (&wgcontext.device, vec![circle_vertex], RECT_INDEX);
        cricle_buffer
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::POLYGON
    }
}

impl ShapeBuffer for Polygon {
    fn to_buffer(&self, wgcontext: &WGContext, color: RGBA) -> VertexBuffer {
        let circle_vertex
            = PolygonVertex::new(&self.point, self.edge, color);
        let cricle_buffer = VertexBuffer::create_vertex_buf::<PolygonVertex>
            (&wgcontext.device, vec![circle_vertex], RECT_INDEX);
        cricle_buffer
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::POLYGON
    }
}