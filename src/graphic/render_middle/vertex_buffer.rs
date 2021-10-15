use std::borrow::BorrowMut;

use bytemuck::Pod;
use wgpu::{Device, RenderPipeline};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::graphic::base::ShapeType;
use crate::graphic::render_middle::render_function::RenderUtil;
use crate::graphic::render_middle::texture::GTexture;

/// 渲染顶点缓冲结构体
#[derive(Debug)]
pub struct VertexBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    // pub shape_type: ShapeType,
}

pub const RECT_INDEX: &[u16; 4] = &[0, 2, 1, 3];
pub const RECT_LINE_INDEX: &[u16; 5] = &[0, 1, 3, 2, 0];

impl<'a> VertexBuffer {
    pub fn create_vertex_buf<V>(device: &Device, vect: Vec<V>
                                , indices: &'a [u16],
                                // , shape_type: ShapeType,
    ) -> Self
        where V: Pod
    {
        log::info!("----create wgpu buffer----");
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
            // shape_type,
        }
    }

    pub fn render(&'a self, render_utils: &mut RenderUtil, shape_type: ShapeType) {
        let pipeline = render_utils.pipeline.get_pipeline(shape_type).unwrap();
        let mut render_pass = render_utils.create_render_pass();
        self.render_shape(render_pass.borrow_mut(), pipeline)
    }

    pub fn render_t(&'a self, render_utils: &mut RenderUtil,
                    texture_state: &'a GTexture) {
        let pipeline = render_utils.pipeline.get_pipeline(ShapeType::TEXTURE).unwrap();
        let mut render_pass = render_utils.create_render_pass();
        self.render_texture(render_pass.borrow_mut(), texture_state, pipeline)
    }

    #[deprecated]
    fn render_shape(&'a self, render_pass: &mut wgpu::RenderPass<'a>,
                    shape_pipeline: &'a RenderPipeline) {
        render_pass.set_pipeline(shape_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    fn render_texture(&'a self, render_pass: &mut wgpu::RenderPass<'a>,
                      texture_state: &'a GTexture,
                      render_pipeline: &'a RenderPipeline) {
        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, &texture_state.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    pub fn render_g_texture(&'a self,
                            render_pass: &mut wgpu::RenderPass<'a>,
                            render_pipeline: &'a RenderPipeline,
                            g_texture: &'a GTexture)
    {
        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, &g_texture.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
