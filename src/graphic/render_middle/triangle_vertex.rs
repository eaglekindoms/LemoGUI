use wgpu::*;

use crate::device::WGContext;
use crate::graphic::base::*;
use crate::graphic::render_middle::vertex_buffer::VertexBuffer;
use crate::graphic::render_middle::vertex_buffer_layout::VertexLayout;

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

impl VertexLayout for PointVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PointVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
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
    pub fn from_shape_to_vector(wgcontext: &WGContext, points: &Vec<Point<f32>>, color: RGBA) -> VertexBuffer {
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
            (&wgcontext.device, vect, indices.as_slice());
        point_buffer
    }
}