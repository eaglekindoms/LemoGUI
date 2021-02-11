use crate::backend::global_setting::GlobalState;
use crate::backend::shape::*;
use wgpu::util::{DeviceExt, BufferInitDescriptor};

pub struct VertexBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl<'a> VertexBuffer {
    pub fn default(global_state: &'a GlobalState, rect: &'a Rectangle, indices: &'a [u16], test_color: RGBA) -> Self {
        let vect = rect.to_buff(global_state.sc_desc.width, global_state.sc_desc.height, test_color);
        let vertex_buffer = global_state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vect.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = global_state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
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
    #[deprecated]
    pub fn create_shape_vertex_buf(global_state: &'a GlobalState, rect: &'a Rectangle) -> Self {
        let test_color = RGBA([0.5, 0.0, 0.5, 0.5]);
        let indices: &[u16] = &[0, 2, 1, 3];
        Self::default(global_state, rect, indices, test_color)
    }

    pub fn create_background_buf(global_state: &'a GlobalState, rect: &'a Rectangle, color: RGBA) -> Self {
        let indices: &[u16] = &[0, 2, 1, 3];
        Self::default(global_state, rect, indices, color)
    }
    #[deprecated]
    pub fn create_border_vertex_buf(global_state: &'a GlobalState, rect: &'a Rectangle) -> Self {
        let test_color = RGBA([0.5, 0.0, 0.5, 1.0]);
        let indices: &[u16] = &[0, 1, 3, 2, 0];
        Self::default(global_state, rect, indices, test_color)
    }

    pub fn create_border_buf(global_state: &'a GlobalState, rect: &'a Rectangle, color: RGBA) -> Self {
        let test_color = RGBA([0.5, 0.0, 0.5, 1.0]);
        let indices: &[u16] = &[0, 1, 3, 2, 0];
        Self::default(global_state, rect, indices, color)
    }

    pub fn create_tex_vertex_buf(global_state: &'a GlobalState, rect: &'a Rectangle) -> Self {
        let vect = rect.to_tex(global_state.sc_desc.width, global_state.sc_desc.height);

        let indices: &[u16] = &[0, 2, 1, 3];
        let vertex_buffer = global_state.device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some("Buffer"),
                contents: bytemuck::cast_slice(vect.as_slice()),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let index_buffer = global_state.device.create_buffer_init(
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
