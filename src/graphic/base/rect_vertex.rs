use wgpu::*;

use crate::graphic::render_middle::pipeline_state::Shader;
use crate::graphic::render_middle::vertex_buffer::{RECT_INDEX, VertexBuffer};
use crate::graphic::render_middle::vertex_buffer_layout::VertexInterface;
use crate::graphic::style::{Bordering, Rounding, Style};
use crate::graphic::base::shape::Rectangle;

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

impl VertexInterface for RectVertex {
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
                    offset: 4 * 2,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 4 * (2 + 2),
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                }, wgpu::VertexAttribute {
                    offset: 4 * (2 + 2 + 4),
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 4 * (2 + 2 + 4 + 4),
                    shader_location: 4,
                    format: wgpu::VertexFormat::Uint32x2,
                },
            ],
        }
    }

    fn set_shader(device: &Device) -> Shader {
        let vs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../../shader_c/round_rect.vert.spv"));
        let fs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../../shader_c/round_rect.frag.spv"));

        Shader {
            vs_module,
            fs_module,
        }
    }
}

impl RectVertex {
    pub fn from_shape_to_vector(device: &Device, sc_desc: &wgpu::SwapChainDescriptor, rect: &Rectangle, style: &Style) -> VertexBuffer {
        let (t_x, t_y, t_w, t_h) =
            rect.get_coord(sc_desc.width, sc_desc.height);
        let border_color;
        let frame_color;
        let is_round;
        let is_border;

        match style.get_border() {
            Bordering::Border(color) => {
                border_color = color.to_f32();
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
        frame_color = style.get_background_color().to_f32();

        let vect = vec![
            RectVertex {
                size: [t_w, t_h],
                position: [t_w / 2.0 + t_x, t_y - t_h / 2.0],
                border_color,
                frame_color,
                is_round_or_border: [is_round, is_border],
            }
        ];
        let back_buffer =
            VertexBuffer::create_vertex_buf::<RectVertex>
                (device, vect, RECT_INDEX);
        back_buffer
    }
}