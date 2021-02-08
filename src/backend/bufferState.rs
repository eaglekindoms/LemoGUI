use crate::backend::globeSetting::GlobeState;
use crate::backend::shape::*;
use wgpu::util::DeviceExt;
use crate::backend::mywgpu;
use wgpu::{BindGroupLayout, BindGroup};
use crate::backend::font::draw_image;

pub struct VertexBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl<'a> VertexBuffer {
    pub fn default(globe_state: &'a GlobeState, rect: &'a Rectangle, indices: &'a [u16], test_color: RGBA) -> Self {
        let vect = rect.to_buff(globe_state.sc_desc.width, globe_state.sc_desc.height, test_color);
        let vertex_buffer = globe_state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vect.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = globe_state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let num_indices = indices.len() as u32;
        Self {
            vertex_buffer,
            num_indices,//11
            index_buffer,
        }
    }
    pub fn create_shape_vertex_buf(globe_state: &'a GlobeState, rect: &'a Rectangle) -> Self {
        let test_color = RGBA([0.5, 0.0, 0.5, 0.5]);
        let indices: &[u16] = &[0, 2, 1, 3];
        Self::default(globe_state, rect, indices, test_color)
    }

    pub fn create_border_vertex_buf(globe_state: &'a GlobeState, rect: &'a Rectangle) -> Self {
        let test_color = RGBA([0.5, 0.0, 0.5, 1.0]);
        let indices: &[u16] = &[0, 1, 3, 2, 0];
        Self::default(globe_state, rect, indices, test_color)
    }

    pub fn create_tex_vertex_buf(globe_state: &'a GlobeState, rect: &'a Rectangle) -> Self {
        let vect = rect.to_tex(globe_state.sc_desc.width, globe_state.sc_desc.height);

        let indices: &[u16] = &[0, 2, 1, 3];
        let vertex_buffer = globe_state.device
            .create_buffer_init(&mywgpu::description::
            create_buffer_init_descriptor(
                bytemuck::cast_slice(vect.as_slice()), wgpu::BufferUsage::VERTEX)
            );
        let index_buffer = globe_state.device.create_buffer_init(
            &mywgpu::description::create_buffer_init_descriptor(
                bytemuck::cast_slice(indices), wgpu::BufferUsage::INDEX)
        );
        let num_indices = indices.len() as u32;
        Self {
            vertex_buffer,
            num_indices,//11
            index_buffer,
        }
    }
}


pub struct TextureState {
    pub texture_bind_group_layout: BindGroupLayout,
    pub diffuse_bind_group: BindGroup,
}

pub struct TextureBuffer<'a> {
    pub x: u32,
    pub y: u32,
    pub buf: &'a [u8],
}

impl<'a> TextureState {
    pub fn default(globe_state: &'a GlobeState, texture_buf: &'a TextureBuffer) -> Self {
        let texture_size = mywgpu::texture::create_texture_size(texture_buf.x, texture_buf.y);
        let diffuse_texture = globe_state.device.create_texture(
            &mywgpu::texture::create_texture_descriptor(&texture_size)
        );
        globe_state.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            mywgpu::texture::create_texture_copy_view(&diffuse_texture),
            // The actual pixel data
            // diffuse_rgba,
            texture_buf.buf,
            // The layout of the texture
            mywgpu::texture::create_texture_data_layout(texture_buf.x, texture_buf.y),
            texture_size,
        );

        /// 默认纹理渲染配置
        let diffuse_texture_view = diffuse_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = globe_state.device.create_sampler(&mywgpu::description::create_sample_descriptor());
        let texture_bind_group_layout = globe_state.device.create_bind_group_layout(
            &mywgpu::description::create_bind_group_layout_descriptor()
        );
        /// 描述纹理顶点数据布局,用于着色器识别数据
        let diffuse_bind_group = globe_state.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        Self {
            texture_bind_group_layout,
            diffuse_bind_group,
        }
    }

    pub fn create_texture_group(globe_state: &'a GlobeState) -> Self {
        let text = "hello button";
        let (x, y, buf) = draw_image(45.0,text);
        let texture_buf = TextureBuffer { x, y, buf: buf.as_slice() };
        Self::default(globe_state, &texture_buf)
    }
}
