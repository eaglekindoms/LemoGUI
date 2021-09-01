use wgpu::*;

use crate::graphic::base::color::RGBA;
use crate::graphic::base::shape::Rectangle;
use crate::graphic::render_middle::pipeline_state::Shader;
use crate::graphic::render_middle::vertex_buffer_layout::VertexLayout;
use crate::graphic::style::{Bordering, Rounding};

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

impl VertexLayout for RectVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectVertex>() as wgpu::BufferAddress,
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
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                }, wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Uint32x2,
                },
            ],
        }
    }

    fn get_shader(device: &Device) -> ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("round_rect shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader_c/round_rect.wgsl")),
            )),
            flags: Default::default(),
        })
    }
}

impl RectVertex {
    pub fn new(rect: &Rectangle, sc_desc: &wgpu::SwapChainDescriptor, color: RGBA) -> RectVertex {
        let (t_x, t_y, t_w, t_h) =
            rect.get_coord(sc_desc.width, sc_desc.height);
        let mut border_color = [0.0, 0.0, 0.0, 0.0];
        let frame_color = color.to_vec();
        let mut is_round = 0;
        let mut is_border = 0;
        if let Some(style) = rect.style {
            match style.get_border() {
                Bordering::Border(color) => {
                    border_color = color.to_vec();
                    is_border = 1;
                }
                Bordering::NoBorder => {
                    border_color = [0.0, 0.0, 0.0, 0.0];
                    is_border = 0;
                }
            }
            match style.get_round() {
                Rounding::Round => is_round = 1,
                Rounding::NoRound => is_round = 0,
            }
        }
        RectVertex {
            size: [t_w, t_h],
            position: [t_w / 2.0 + t_x, t_y - t_h / 2.0],
            border_color,
            frame_color,
            is_round_or_border: [is_round, is_border],
        }
    }
}