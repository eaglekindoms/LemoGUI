use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::texture_buffer::TextureBuffer;
use crate::graphic::render_middle::vertex_buffer::VertexBuffer;

/// 组件渲染中间结构体
#[derive(Debug)]
pub struct RenderGraph {
    pub vertex_buffer: VertexBuffer,
    pub back_buffer: VertexBuffer,
    pub context_buffer: TextureBuffer,
}

impl RenderGraph {
    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,
                    glob_pipeline: &'a PipelineState) {
        self.back_buffer.render_shape(render_pass, &glob_pipeline.round_shape_pipeline);
        self.vertex_buffer.render_texture(render_pass, &self.context_buffer, &glob_pipeline.texture_pipeline);
    }
}