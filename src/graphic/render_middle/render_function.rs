use wgpu::RenderPipeline;

use crate::graphic::base::image2d::TextureBuffer;
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::vertex_buffer::VertexBuffer;

/// 组件渲染中间结构体
pub struct RenderGraph {
    pub vertex_buffer: VertexBuffer,
    pub back_buffer: VertexBuffer,
    pub border_buffer: VertexBuffer,
    pub context_buffer: TextureBuffer,
}

pub fn render_shape<'a>(render_pass: &mut wgpu::RenderPass<'a>,
                        shape_pipeline: &'a RenderPipeline, shape_vertex_buffer: &'a VertexBuffer) {
    render_pass.set_pipeline(shape_pipeline);
    render_pass.set_vertex_buffer(0, shape_vertex_buffer.vertex_buffer.slice(..));
    render_pass.set_index_buffer(shape_vertex_buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..shape_vertex_buffer.num_indices, 0, 0..1);
}

pub fn render_texture<'a>(render_pass: &mut wgpu::RenderPass<'a>, texture_state: &'a TextureBuffer,
                          render_pipeline: &'a RenderPipeline, vertex_buffer: &'a VertexBuffer) {
    render_pass.set_pipeline(&render_pipeline);
    render_pass.set_bind_group(0, &texture_state.diffuse_bind_group, &[]); // NEW!
    render_pass.set_vertex_buffer(0, vertex_buffer.vertex_buffer.slice(..));
    render_pass.set_index_buffer(vertex_buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..vertex_buffer.num_indices, 0, 0..1);
}

impl RenderGraph {
    pub fn draw_rect<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,
                         glob_pipeline: &'a PipelineState, focused: bool) {
        render_shape(render_pass, &glob_pipeline.shape_pipeline, &self.back_buffer);
        render_shape(render_pass, &glob_pipeline.border_pipeline, &self.border_buffer);
        render_texture(render_pass, &self.context_buffer, &glob_pipeline.texture_pipeline, &self.vertex_buffer);

    }

    pub fn draw_round_rect<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,
                               glob_pipeline: &'a PipelineState, focused: bool) {
        render_shape(render_pass, &glob_pipeline.round_shape_pipeline, &self.back_buffer);
        // render_shape(render_pass, &glob_pipeline.round_shape_pipeline, &self.border_buffer);
        render_texture(render_pass, &self.context_buffer, &glob_pipeline.texture_pipeline, &self.vertex_buffer);

    }
}