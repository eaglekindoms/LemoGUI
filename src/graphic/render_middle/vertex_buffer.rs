use bytemuck::Pod;
use wgpu::Device;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::graphic::base::*;
use crate::graphic::base::color::RGBA;
use crate::graphic::base::point2d::PointVertex;
use crate::graphic::base::rectangle::{Rectangle, RectVertex};
use crate::graphic::render_middle::vertex_buffer_layout::VertexBufferLayout;

pub struct VertexBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl<'a> VertexBuffer {
    pub fn create_vertex_buf<V>(device: &Device,
                                sc_desc: &wgpu::SwapChainDescriptor,
                                rect: &'a Rectangle
                                , indices: &'a [u16]
                                , test_color: RGBA,
    ) -> Self
        where V: VertexBufferLayout + Pod
    {
        let vect = V::from_shape_to_vector(rect, sc_desc, test_color);

        let vertex_buffer = device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some("Buffer"),
                contents: bytemuck::cast_slice(vect.as_slice()),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let index_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("Buffer"),
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
}
