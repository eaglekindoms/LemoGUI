use wgpu::*;

use crate::device::display_window::WGContext;
use crate::graphic::base::color::RGBA;
use crate::graphic::base::shape::*;
use crate::graphic::render_middle::pipeline_state::Shader;
use crate::graphic::render_middle::vertex_buffer::VertexBuffer;
use crate::graphic::render_middle::vertex_buffer_layout::VertexInterface;

/// 多边形顶点数据布局结构体
#[repr(C)]
#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointVertex {
    pub position: Point,
    pub color: RGBA,
}

impl PointVertex {
    pub fn new(x: f32, y: f32, color: RGBA) -> Self {
        log::info!("create the PointVertex obj");
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
            .create_shader_module(&wgpu::include_spirv!("../../../shader_c/triangle.vert.spv"));
        let fs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../../shader_c/triangle.frag.spv"));

        Shader {
            vs_module,
            fs_module,
        }
    }
}

impl PointVertex {
    pub fn from_shape_to_vector(wgcontext: &WGContext, points: &Vec<Point>, color: RGBA) -> VertexBuffer {
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