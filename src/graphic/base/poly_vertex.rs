use wgpu::*;

use crate::graphic::base::color::*;
use crate::graphic::base::shape::*;
use crate::graphic::render_middle::pipeline_state::Shader;
use crate::graphic::render_middle::vertex_buffer::{RECT_INDEX, VertexBuffer};
use crate::graphic::render_middle::vertex_buffer_layout::VertexLayout;

/// 圆形顶点数据布局结构体
/// 顶点顺序为左下开始逆时针排序
#[repr(C)]
#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PolygonVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub radius: f32,
    pub edge: u32,
}

impl PolygonVertex {
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

impl VertexLayout for PolygonVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PolygonVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
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

    fn set_shader(device: &Device) -> Shader {
        let vs_module = device
            .create_shader_module(&wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader_c/polygon.vert.spv")));
        let fs_module = device
            .create_shader_module(&wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader_c/polygon.frag.spv")));

        Shader {
            vs_module,
            fs_module,
        }
    }
}

impl PolygonVertex {
    pub fn from_shape_to_vector(device: &Device, sc_desc: &wgpu::SwapChainDescriptor, circle: PolygonVertex) -> VertexBuffer {
        let vect = vec![circle];
        let cricle_buffer = VertexBuffer::create_vertex_buf::<PolygonVertex>
            (device, vect, RECT_INDEX);
        cricle_buffer
    }
}