use wgpu::{CommandEncoder, SwapChainTexture};

use crate::graphic::base::shape::ShapeType;
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

/// 渲染工具封装结构体
/// 作用：省事
#[derive(Debug)]
pub struct RenderUtil {
    pub encoder: CommandEncoder,
    pub target: SwapChainTexture,
}

impl RenderGraph {
    pub fn draw(&self, render_utils: &mut RenderUtil,
                glob_pipeline: &PipelineState) {
        self.back_buffer.render(render_utils,
                                glob_pipeline, ShapeType::ROUND);
        let mut render_pass = render_utils.create_render_pass();
        self.vertex_buffer.render_texture(&mut render_pass,
                                          &self.context_buffer,
                                          &glob_pipeline.get_pipeline(ShapeType::TEXTURE).unwrap());
    }
}

impl RenderUtil {
    /// 创建渲染中间变量
    pub fn create_render_pass<'a>(&'a mut self) -> wgpu::RenderPass<'a> {
        let render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &self.target.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        render_pass
    }
}