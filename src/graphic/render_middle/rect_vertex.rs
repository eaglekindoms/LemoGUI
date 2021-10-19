use wgpu::*;

use crate::graphic::base::*;
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

const ATTRS: [VertexAttribute; 5] = wgpu::vertex_attr_array![
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
            attributes: &ATTRS,
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