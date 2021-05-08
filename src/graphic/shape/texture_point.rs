use wgpu::*;
use wgpu::util::DeviceExt;

use crate::graphic::render_type::pipeline_state::PipelineState;
use crate::graphic::render_type::texture_state::TextureState;
use crate::graphic::shape::rectangle::{Rectangle, TransferVertex};

/// 2D纹理顶点数据
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexturePoint {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

impl TexturePoint {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TexturePoint>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
    }
}

pub struct TextState {
    // pub text_pipeline: RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl<'a> TextState {
    pub fn new(device: &Device, sc_desc: &wgpu::SwapChainDescriptor, rect: &'a Rectangle) -> Self {
        let vect = rect.to_tex(sc_desc.width, sc_desc.height);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vect.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&[0, 2, 1, 3]),
            usage: wgpu::BufferUsage::INDEX,
        });
        log::info!("create the TextState obj");
        Self {
            // text_pipeline,
            vertex_buffer,
            index_buffer,
        }
    }
}

