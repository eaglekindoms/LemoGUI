use wgpu::*;

use crate::graphic::base::color::RGBA;
use crate::graphic::base::shape::*;
use crate::graphic::render_middle::pipeline_state::Shader;
use crate::graphic::render_middle::vertex_buffer::{VertexBuffer, RECT_INDEX};
use crate::graphic::render_middle::vertex_buffer_layout::VertexInterface;

/// 圆形顶点数据布局结构体
#[repr(C)]
#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CircleVertex {
    pub position: Point,
    pub color: RGBA,
    pub radius: f32,
}

impl CircleVertex {
    pub fn new(x: f32, y: f32, r: f32, color: RGBA) -> Self {
        log::info!("create the CircleVertex obj");
        Self {
            position: Point { x, y },
            color,
            radius: r,
        }
    }
}

impl VertexInterface for CircleVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CircleVertex>() as wgpu::BufferAddress,
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
            ],
        }
    }

    fn set_shader(device: &Device) -> Shader {
        let vs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../../shader_c/circle.vert.spv"));
        let fs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../../shader_c/circle.frag.spv"));

        Shader {
            vs_module,
            fs_module,
        }
    }
}

impl CircleVertex {
    pub fn from_shape_to_vector(device: &Device, sc_desc: &wgpu::SwapChainDescriptor, circle: CircleVertex) -> VertexBuffer {
        let vect=vec![circle];
        let cricle_buffer = VertexBuffer::create_vertex_buf::<CircleVertex>
            (device, vect,RECT_INDEX);
        cricle_buffer
    }
}