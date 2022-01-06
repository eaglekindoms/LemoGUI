use std::borrow::BorrowMut;

use bytemuck::Pod;
use wgpu::{Device, RenderPipeline};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::backend::wgpu_impl::*;
use crate::graphic::base::ShapeType;

/// 渲染顶点缓冲结构体
#[derive(Debug)]
pub struct VertexBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub shape_type: ShapeType,
}

pub const RECT_INDEX: &[u16; 4] = &[0, 2, 1, 3];
pub const RECT_LINE_INDEX: &[u16; 5] = &[0, 1, 3, 2, 0];

impl<'a> VertexBuffer {
    pub fn create_vertex_buf<V>(device: &Device, vect: Vec<V>
                                , indices: &'a [u16]) -> Self
        where V: Pod + VertexLayout
    {
        log::info!("----create wgpu buffer----");
        let shape_type = V::get_shape_type();
        let vertex_buffer = device
            .create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vect.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let num_indices = indices.len() as u32;
        Self {
            vertex_buffer,
            num_indices,
            index_buffer,
            shape_type,
        }
    }

    pub fn render(&'a self, render_utils: &mut RenderUtil,
                  texture_state: Option<&'a TextureBufferData>) {
        let pipeline =
            render_utils.context.glob_pipeline.get_pipeline(self.shape_type).unwrap();
        let mut render_pass =
            create_render_pass(&mut render_utils.encoder, &render_utils.view);
        render_pass.set_pipeline(&pipeline);
        if let Some(texture_buffer) = texture_state {
            render_pass.set_bind_group(0, &texture_buffer.uniform, &[]);
        }
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

/// 创建渲染中间变量
fn create_render_pass<'a>(encoder: &'a mut wgpu::CommandEncoder, target: &'a wgpu::TextureView) -> wgpu::RenderPass<'a> {
    let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[wgpu::RenderPassColorAttachment {
            view: target,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        }],
        depth_stencil_attachment: None,
    });
    render_pass
}