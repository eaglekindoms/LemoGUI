use wgpu::RenderPipeline;
use crate::backend::buffer_state::{VertexBuffer, TextureState};
use crate::backend::pipeline_state::PipelineState;
use crate::widget::button::ButtonGraph;

pub fn render_shape<'a>(render_pass: &mut wgpu::RenderPass<'a>,
                        shape_pipeline: &'a RenderPipeline, shape_vertex_buffer: &'a VertexBuffer) {
    render_pass.set_pipeline(shape_pipeline);
    render_pass.set_vertex_buffer(0, shape_vertex_buffer.vertex_buffer.slice(..));
    render_pass.set_index_buffer(shape_vertex_buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..shape_vertex_buffer.num_indices, 0, 0..1);
}

pub fn render_texture<'a>(render_pass: &mut wgpu::RenderPass<'a>, texture_state: &'a TextureState,
                          render_pipeline: &'a RenderPipeline, vertex_buffer: &'a VertexBuffer) {
    render_pass.set_pipeline(&render_pipeline);
    render_pass.set_bind_group(0, &texture_state.diffuse_bind_group, &[]); // NEW!
    render_pass.set_vertex_buffer(0, vertex_buffer.vertex_buffer.slice(..));
    render_pass.set_index_buffer(vertex_buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..vertex_buffer.num_indices, 0, 0..1);
}

pub fn render_button<'a>(render_pass: &mut wgpu::RenderPass<'a>,
                         glob_pipeline: &'a PipelineState, button_graph: &'a ButtonGraph, focused: bool) {
    render_shape(render_pass, &glob_pipeline.shape_pipeline, &button_graph.back_buffer);
    render_shape(render_pass, &glob_pipeline.border_pipeline, &button_graph.boder_buffer);
    render_texture(render_pass, &button_graph.font_buffer, &glob_pipeline.render_pipeline, &button_graph.vertex_buffer);
    if focused {
        render_shape(render_pass, &glob_pipeline.shape_pipeline, &button_graph.hover_buffer);
    }
}