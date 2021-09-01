use wgpu::*;

use crate::graphic::base::color::*;
use crate::graphic::base::shape::*;
use crate::graphic::render_middle::vertex_buffer_layout::VertexLayout;

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

impl VertexLayout for CircleVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CircleVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
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
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
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