use wgpu::RenderPipeline;
use crate::backend::bufferState::{VertexBuffer, TextureState};

pub fn render_shape<'a>(render_pass: &mut wgpu::RenderPass<'a>,
                        shape_pipeline: &'a RenderPipeline, shape_vertex_buffer: &'a VertexBuffer) {
    render_pass.set_pipeline(shape_pipeline);
    render_pass.set_vertex_buffer(0, shape_vertex_buffer.vertex_buffer.slice(..));
    render_pass.set_index_buffer(shape_vertex_buffer.index_buffer.slice(..));
    render_pass.draw_indexed(0..shape_vertex_buffer.num_indices, 0, 0..1);
}

pub fn render_texture<'a>(render_pass: &mut wgpu::RenderPass<'a>, texture_state: &'a TextureState,
                          render_pipeline: &'a RenderPipeline, vertex_buffer: &'a VertexBuffer) {
    render_pass.set_pipeline(&render_pipeline);
    render_pass.set_bind_group(0, &texture_state.diffuse_bind_group, &[]); // NEW!
    render_pass.set_vertex_buffer(0, vertex_buffer.vertex_buffer.slice(..));
    render_pass.set_index_buffer(vertex_buffer.index_buffer.slice(..));
    render_pass.draw_indexed(0..vertex_buffer.num_indices, 0, 0..1);
}