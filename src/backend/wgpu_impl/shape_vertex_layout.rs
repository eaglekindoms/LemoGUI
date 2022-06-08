use wgpu::*;

use crate::backend::wgpu_impl::*;
use crate::graphic::base::*;
use crate::graphic::style::{Bordering, Rounding, Style};

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

    fn get_shape_type() -> ShapeType {
        ShapeType::Circle
    }

    fn get_shader(device: &Device) -> ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("circle shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "./shader/circle.wgsl"
            ))),
        })
    }
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

    fn get_shape_type() -> ShapeType {
        ShapeType::ROUND
    }

    fn get_shader(device: &Device) -> ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("round_rect shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "./shader/round_rect.wgsl"
            ))),
        })
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

    fn get_shape_type() -> ShapeType {
        ShapeType::POINT
    }

    fn get_shader(device: &Device) -> ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("triangle shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "./shader/triangle.wgsl"
            ))),
        })
    }
}

pub fn from_shape_to_vector(
    gpu_context: &WGPUContext,
    points: &Vec<Point<f32>>,
    color: RGBA,
) -> VertexBuffer {
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
    let point_buffer = VertexBuffer::create_vertex_buf::<PointVertex>(
        &gpu_context.device,
        vect,
        indices.as_slice(),
    );
    point_buffer
}
