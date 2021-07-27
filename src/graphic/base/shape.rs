
use crate::device::display_window::WGContext;
use crate::graphic::base::circle_vertex::CircleVertex;
use crate::graphic::base::poly_vertex::PolyVertex;
use crate::graphic::base::rect_vertex::RectVertex;
use crate::graphic::render_middle::vertex_buffer::{RECT_INDEX, VertexBuffer};
use crate::graphic::style::Style;

/// 图形类型
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ShapeType {
    TEXTURE = 0,
    ROUND = 1,
    BORDER = 2,
    POLYGON = 3,
    CIRCLE = 4,
}

/// 二维顶点结构体
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

#[derive(Debug, Default, Copy, Clone)]
pub struct Circle {
    pub position: Point,
    pub radius: f32,
}

#[derive(Debug)]
pub struct Polygon {
    pub points: Vec<Point>,
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
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon {
            points
        }
    }
}

pub trait ShapeBuffer {
    fn to_buffer(&self, wgcontext: &WGContext, style: &Style) -> VertexBuffer;
    fn get_type(&self) -> ShapeType;
}

impl ShapeBuffer for Rectangle {
    fn to_buffer(&self, wgcontext: &WGContext, style: &Style) -> VertexBuffer {
        RectVertex::from_shape_to_vector
            (&wgcontext.device, &wgcontext.sc_desc, &self, style)
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::ROUND
    }
}

impl ShapeBuffer for Circle {
    fn to_buffer(&self, wgcontext: &WGContext, style: &Style) -> VertexBuffer {
        let circle_vertex
            = CircleVertex::new(self.position.x, self.position.y, self.radius, style.get_background_color());
        let cricle_buffer = VertexBuffer::create_vertex_buf::<CircleVertex>
            (&wgcontext.device, vec![circle_vertex], RECT_INDEX);
        cricle_buffer
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::CIRCLE
    }
}

impl ShapeBuffer for Polygon {
    fn to_buffer(&self, wgcontext: &WGContext, style: &Style) -> VertexBuffer {
        PolyVertex::from_shape_to_vector(wgcontext, &self.points, style.get_background_color())
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::POLYGON
    }
}