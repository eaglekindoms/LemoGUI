use wgpu::*;

use crate::graphic::base::color::RGBA;
use crate::graphic::base::point2d::Point;
use crate::graphic::render_middle::pipeline_state::Shader;
use crate::graphic::render_middle::vertex_buffer_layout::VertexInterface;
use crate::graphic::style;
use crate::graphic::style::Bordering;

/// 矩形结构体
#[derive(Debug)]
pub struct Rectangle {
    pub position: Point,
    pub width: u32,
    pub height: u32,
    style: style::Style,
}

#[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RectVertex {
    pub size: [f32; 2],
    pub position: [f32; 2],
    pub border_color: [f32; 4],
    pub frame_color: [f32; 4],
    pub border_radius: f32,
    pub border_width: f32,
}


impl Rectangle {
    pub fn new(x: f32, y: f32, w: u32, h: u32) -> Rectangle {
        log::info!("create the Rectangle obj");
        Rectangle {
            position: Point { x: x, y: y },
            width: w,
            height: h,
            style: style::Style::default(),
        }
    }
    pub fn set_border(&mut self, border: Bordering) {
        self.style = style::Style::set_border(&mut self.style, border);
    }

    pub fn get_coord(&self, w_width: u32, w_height: u32) -> (f32, f32, f32, f32) {
        (2.0 * self.position.x as f32 / w_width as f32 - 1.0,
         1.0 - 2.0 * self.position.y as f32 / w_height as f32,
         2.0 * self.width as f32 / w_width as f32,
         2.0 * self.height as f32 / w_height as f32)
    }
}

impl VertexInterface for RectVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 4 * 2,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 4 * (2 + 2),
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                }, wgpu::VertexAttribute {
                    offset: 4 * (2 + 2 + 4),
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 4 * (2 + 2 + 4 + 4),
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: 4 * (2 + 2 + 4 + 4 + 1),
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32,
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

    fn from_shape_to_vector(rect: &Rectangle, sc_desc: &wgpu::SwapChainDescriptor, test_color: RGBA) -> Vec<Self> {
        let (t_x, t_y, t_w, t_h) =
            rect.get_coord(sc_desc.width, sc_desc.height);

        vec![RectVertex {
            size: [t_w, t_h],
            position: [t_w / 2.0 + t_x, t_h / 2.0 + t_y],
            border_color: [0.0, 0.0, 0.0, 1.0],
            frame_color: test_color.0,
            border_radius: 0.05,
            border_width: 0.005,
        }]
    }
}