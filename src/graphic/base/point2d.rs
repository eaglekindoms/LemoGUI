use wgpu::{Device, PipelineLayout, PrimitiveTopology, RenderPipeline, VertexBufferLayout, VertexState};

use crate::graphic::base::color::RGBA;
use crate::graphic::base::rectangle::Rectangle;
use crate::graphic::render_middle::pipeline_state::create_render_pipeline;
use crate::graphic::render_middle::shader::Shader;
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

    fn set_pipeline_layout(device: &Device) -> PipelineLayout {
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        return render_pipeline_layout;
    }

    fn set_fill_topology() -> PrimitiveTopology {
        wgpu::PrimitiveTopology::LineStrip
    }

    fn from_shape_to_vector<'a>(rect: &'a Rectangle, sc_desc: &wgpu::SwapChainDescriptor, test_color: RGBA) -> Vec<Self> {
        let (t_x, t_y, t_w, t_h) =
            rect.get_coord(sc_desc.width, sc_desc.height);

        let vect: Vec<PointVertex> = vec![
            PointVertex::new(t_x, t_y, test_color), // 左上
            PointVertex::new(t_x + t_w, t_y, test_color), // 右上
            PointVertex::new(t_x, t_y - t_h, test_color), // 左下
            PointVertex::new(t_x + t_w, t_y - t_h, test_color), // 右下
        ];
        return vect;
    }
}