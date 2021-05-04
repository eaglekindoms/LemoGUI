use wgpu::RenderPipeline;

use crate::graphic::render_type::buffer_state::VertexBuffer;
use crate::graphic::render_type::pipeline_state::PipelineState;
use crate::graphic::render_type::texture_state::TextureState;

/// 组件渲染中间结构体
pub struct RenderGraph {
    pub vertex_buffer: VertexBuffer,
    pub back_buffer: VertexBuffer,
    pub hover_buffer: Option<VertexBuffer>,
    pub border_buffer: VertexBuffer,
    pub context_buffer: TextureState,
}

fn render_shape<'a>(render_pass: &mut wgpu::RenderPass<'a>,
                    shape_pipeline: &'a RenderPipeline, shape_vertex_buffer: &'a VertexBuffer) {
    render_pass.set_pipeline(shape_pipeline);
    render_pass.set_vertex_buffer(0, shape_vertex_buffer.vertex_buffer.slice(..));
    render_pass.set_index_buffer(shape_vertex_buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..shape_vertex_buffer.num_indices, 0, 0..1);
}

fn render_texture<'a>(render_pass: &mut wgpu::RenderPass<'a>, texture_state: &'a TextureState,
                      render_pipeline: &'a RenderPipeline, vertex_buffer: &'a VertexBuffer) {
    render_pass.set_pipeline(&render_pipeline);
    render_pass.set_bind_group(0, &texture_state.diffuse_bind_group, &[]); // NEW!
    render_pass.set_vertex_buffer(0, vertex_buffer.vertex_buffer.slice(..));
    render_pass.set_index_buffer(vertex_buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..vertex_buffer.num_indices, 0, 0..1);
}

#[deprecated]
pub fn render_button<'a>(render_pass: &mut wgpu::RenderPass<'a>,
                         glob_pipeline: &'a PipelineState, button_graph: &'a RenderGraph, focused: bool) {
    render_shape(render_pass, &glob_pipeline.shape_pipeline, &button_graph.back_buffer);
    render_shape(render_pass, &glob_pipeline.border_pipeline, &button_graph.border_buffer);
    render_texture(render_pass, &button_graph.context_buffer, &glob_pipeline.texture_pipeline, &button_graph.vertex_buffer);
    if focused {
        render_shape(render_pass, &glob_pipeline.shape_pipeline, button_graph.hover_buffer.as_ref().unwrap());
    }
}

pub trait Render {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,
                  glob_pipeline: &'a PipelineState, focused: bool) {}
}

impl Render for RenderGraph {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,
                  glob_pipeline: &'a PipelineState, focused: bool) {
        render_shape(render_pass, &glob_pipeline.shape_pipeline, &self.back_buffer);
        render_shape(render_pass, &glob_pipeline.border_pipeline, &self.border_buffer);
        render_texture(render_pass, &self.context_buffer, &glob_pipeline.texture_pipeline, &self.vertex_buffer);
        if focused && !self.hover_buffer.is_none() {
            render_shape(render_pass, &glob_pipeline.shape_pipeline, self.hover_buffer.as_ref().unwrap());
        }
    }
}