use bytemuck::Pod;
use wgpu::{BufferDescriptor, Device};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::graphic::base::color::RGBA;
use crate::graphic::base::rectangle::Rectangle;
use crate::graphic::render_middle::vertex_buffer_layout::VertexInterface;

pub struct VertexBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}
pub const RECT_INDEX: &[u16; 4] =&[0,2,1,3];
pub const RECT_LINE_INDEX: &[u16; 5] =&[0, 1, 3, 2, 0];

impl<'a> VertexBuffer {
    pub fn create_vertex_buf<V>(device: &Device,
                                sc_desc: &wgpu::SwapChainDescriptor,
                                rect: &'a Rectangle
                                , indices: &'a [u16]
                                , test_color: RGBA,
    ) -> Self
        where V: VertexInterface + Pod
    {
        let vect = V::from_shape_to_vector(rect, sc_desc, test_color);
        log::info!("----create wgpu buffer----");
        let vertex_buffer = device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some("vertex Buffer"),
                contents: bytemuck::cast_slice(vect.as_slice()),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let index_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsage::INDEX,
            });
        let num_indices = indices.len() as u32;
        Self {
            vertex_buffer,
            num_indices,
            index_buffer,
        }
    }
}
