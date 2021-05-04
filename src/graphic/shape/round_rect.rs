use wgpu::*;
use wgpu::util::DeviceExt;

use crate::graphic::render_type::pipeline_state::PipelineState;
use crate::graphic::shape::point::{Rectangle, TransferVertex};
use crate::graphic::shape::triangle::RGBA;

const INDICES: &[u16] = &[1, 2, 0, 3];

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RectVertex {
    pub size: [f32; 2],
    pub position: [f32; 2],
    pub border_color: [f32; 4],
    pub frame_color: [f32; 4],
    pub border_radius: f32,
    pub border_width: f32,
}

impl RectVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    offset: 4 * 2,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    offset: 4 * (2 + 2),
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float4,
                }, wgpu::VertexAttribute {
                    offset: 4 * (2 + 2 + 4),
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttribute {
                    offset: 4 * (2 + 2 + 4 + 4),
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float,
                },
                wgpu::VertexAttribute {
                    offset: 4 * (2 + 2 + 4 + 4 + 1),
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float,
                },
            ],
        }
    }
}

const RECT: &[RectVertex] = &[RectVertex {
    size: [0.15, 0.15],
    position: [-0.3, 0.3],
    border_color: [0.0, 0.0, 0.0, 1.0],
    frame_color: [0.3, 0.3, 0.3, 1.0],
    border_radius: 0.02,
    border_width: 0.02,
},
    RectVertex {
        size: [0.25, 0.25],
        position: [0.3, 0.3],
        border_color: [0.0, 0.0, 0.0, 1.0],
        frame_color: [0.3, 0.3, 0.3, 1.0],
        border_radius: 0.03,
        border_width: 0.019,
    },
    RectVertex {
        size: [0.1, 0.35],
        position: [0.3, -0.3],
        border_color: [0.0, 0.0, 0.0, 1.0],
        frame_color: [0.3, 0.3, 0.3, 1.0],
        border_radius: 0.04,
        border_width: 0.005,
    },
    RectVertex {
        size: [0.05, 0.05],
        position: [-0.3, -0.3],
        border_color: [0.0, 0.0, 0.0, 1.0],
        frame_color: [0.3, 0.3, 0.3, 1.0],
        border_radius: 0.01,
        border_width: 0.001,
    }
];

pub struct RectState {
    // pub rect_pipeline: RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl<'a> RectState {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor, rect: &'a Rectangle, color: RGBA) -> Self {
        let vect = rect.to_round(sc_desc.width, sc_desc.height, color);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&[vect]),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::INDEX,
        });
        Self {
            // rect_pipeline,
            vertex_buffer,
            index_buffer,
        }
    }

    // pub fn render(&'a self, render_pipeline: &'a PipelineState, render_pass: &mut wgpu::RenderPass<'a>) {
    //     render_pass.set_pipeline(&render_pipeline.round_shape_pipeline);
    //     render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    //     render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    //     render_pass.draw_indexed(0..4, 0, 0..1);
    // }
}
