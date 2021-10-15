use wgpu::*;

use crate::graphic::base::{Point, Rectangle};
use crate::graphic::render_middle::DEFAULT_BIND_GROUP_LAYOUT;
use crate::graphic::render_middle::vertex_buffer::{RECT_INDEX, VertexBuffer};
use crate::graphic::render_middle::vertex_buffer_layout::VertexLayout;

/// 2D纹理顶点数据布局结构体
#[repr(C)]
#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl VertexLayout for TextureVertex {
    fn set_vertex_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextureVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
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
                },
            ],
        }
    }

    fn get_shader(device: &Device) -> ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("texture shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader_c/image.wgsl")),
            )),
        })
    }

    fn set_pipeline_layout(device: &Device) -> PipelineLayout {
        let texture_bind_group_layout = device.create_bind_group_layout(
            DEFAULT_BIND_GROUP_LAYOUT
        );
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });
        return render_pipeline_layout;
    }
}

impl TextureVertex {
    pub fn new(device: &Device, sc_desc: Point<u32>, rect: &Rectangle) -> VertexBuffer {
        let (t_x, t_y, t_w, t_h) =
            rect.get_coord(sc_desc.x, sc_desc.y);
        let color: [f32; 4];
        match rect.style {
            None => { color = [0.0, 0.0, 0.0, 0.0] }
            Some(style) => { color = style.get_font_color().to_vec(); }
        }
        let vect: Vec<TextureVertex> = vec![
            TextureVertex { position: [t_x, t_y], tex_coords: [t_w, t_h], color }
        ];
        let vertex_buffer = VertexBuffer::create_vertex_buf::<TextureVertex>
            (device, vect, RECT_INDEX);
        vertex_buffer
    }
}