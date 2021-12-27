use wgpu::*;

use crate::backend::wgpu_impl::*;
use crate::device::GPUContext;
use crate::graphic::base::*;

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

const CIRCLE_ATTRS: [VertexAttribute; 4] = wgpu::vertex_attr_array![
                0 => Float32x2,
                1 => Float32x4,
                2 => Float32,
                3 => Uint32];

impl VertexLayout for CircleVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CircleVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &CIRCLE_ATTRS,
        }
    }

    fn get_shader(device: &Device) -> ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("circle shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader_c/circle.wgsl")),
            )),
        })
    }
}


/// 矩形顶点数据布局结构体
#[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RectVertex {
    pub size: [f32; 2],
    pub position: [f32; 2],
    pub border_color: [f32; 4],
    pub frame_color: [f32; 4],
    pub is_round_or_border: [u32; 2],
}

const RECT_ATTRS: [VertexAttribute; 5] = wgpu::vertex_attr_array![
                0 => Float32x2,
                1 => Float32x2,
                2 => Float32x4,
                3 => Float32x4,
                4 => Uint32x2];

impl VertexLayout for RectVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &RECT_ATTRS,
        }
    }

    fn get_shader(device: &Device) -> ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("round_rect shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader_c/round_rect.wgsl")),
            )),
        })
    }
}

impl RectVertex {
    pub fn new(rect: &Rectangle, sc_desc: Point<u32>, color: RGBA) -> RectVertex {
        let (t_x, t_y, t_w, t_h) =
            rect.get_coord(sc_desc.x, sc_desc.y);
        let mut border_color = [0.0, 0.0, 0.0, 0.0];
        let frame_color = color.to_vec();
        let is_round = rect.round;
        let is_border = rect.border;
        if is_border == 1 {
            border_color = BLACK.to_vec();
        }
        // let projection: cgmath::Matrix4<f32> = shape::orthographic_projection(sc_desc.x as f32, sc_desc.y as f32).into();
        // let position: cgmath::Vector4<f32> = cgmath::Vector4::new(rect.position.x, rect.position.y, 0.0, 0.0);
        // let view: cgmath::Vector4<f32> = projection * position;
        RectVertex {
            size: [t_w, t_h],
            position: [t_w / 2.0 + t_x, t_y - t_h / 2.0],
            border_color,
            frame_color,
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

const POINT_ATTRS: [VertexAttribute; 2] = wgpu::vertex_attr_array![
                0 => Float32x2,
                1 => Float32x4 ];

impl VertexLayout for PointVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PointVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &POINT_ATTRS,
        }
    }

    fn get_shader(device: &Device) -> ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("triangle shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader_c/triangle.wgsl")),
            )),
        })
    }
}

impl PointVertex {
    pub fn from_shape_to_vector(gpu_context: &GPUContext, points: &Vec<Point<f32>>, color: RGBA) -> VertexBuffer {
        let vertex_nums = (points.len() - 3) * 2 + points.len();
        let mut vect = Vec::with_capacity(points.len());
        let mut indices = Vec::with_capacity(vertex_nums);
        for i in 0..points.len() {
            vect.push(PointVertex::new(points[i].x, points[i].y, color));
        }
        let mut i = 1u16;
        while i < points.len() as u16 - 1 {
            indices.push(0);
            indices.push(i);
            i = i + 1;
            indices.push(i);
        }
        let point_buffer = VertexBuffer::create_vertex_buf::<PointVertex>
            (&gpu_context.device, vect, indices.as_slice());
        point_buffer
    }
}