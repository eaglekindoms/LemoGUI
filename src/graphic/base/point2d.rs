use wgpu::*;

use crate::graphic::base::color::RGBA;
use crate::graphic::base::rectangle::Rectangle;
use crate::graphic::render_middle::pipeline_state::Shader;
use crate::graphic::render_middle::vertex_buffer::VertexBuffer;
use crate::graphic::render_middle::vertex_buffer_layout::VertexInterface;

/// 二维顶点结构体
#[repr(C)]
#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// 2d图形缓存顶点结构体
#[repr(C)]
#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointVertex {
    pub position: Point,
    pub color: RGBA,
}

impl PointVertex {
    pub fn new(x: f32, y: f32, color: RGBA) -> Self {
        log::info!("create the BufferPoint obj");
        Self {
            position: Point { x, y },
            color,
        }
    }
}

impl VertexInterface for PointVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PointVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
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

    fn set_shader(device: &Device) -> Shader {
        let vs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../../shader_c/rect.vert.spv"));
        let fs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../../shader_c/rect.frag.spv"));

        Shader {
            vs_module,
            fs_module,
        }
    }
}

impl PointVertex {
    pub fn from_shape_to_vector(device: &Device, sc_desc: &wgpu::SwapChainDescriptor, rect: &Rectangle, indices: &[u16], color: RGBA) -> VertexBuffer {
        let (t_x, t_y, t_w, t_h) =
            rect.get_coord(sc_desc.width, sc_desc.height);
        let vect: Vec<PointVertex> = vec![
            PointVertex::new(t_x, t_y, color), // 左上
            PointVertex::new(t_x + t_w, t_y, color), // 右上
            PointVertex::new(t_x, t_y - t_h, color), // 左下
            PointVertex::new(t_x + t_w, t_y - t_h, color), // 右下
        ];
        let point_buffer = VertexBuffer::create_vertex_buf::<PointVertex>
            (device, vect, indices);
        point_buffer
    }
}