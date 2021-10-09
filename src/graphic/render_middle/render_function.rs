use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

use crate::device::wgpu_context::WGContext;
use crate::graphic::base::color::RGBA;
use crate::graphic::render_middle::pipeline_state::PipelineState;

/// 渲染工具封装结构体
/// 作用：省事
#[derive(Debug)]
pub struct RenderUtil<'a> {
    pub encoder: CommandEncoder,
    pub view: TextureView,
    pub context: &'a WGContext,
    pub pipeline: &'a PipelineState,
}

impl<'a> RenderUtil<'a> {
    pub fn new(frame: &SurfaceTexture,
               wgcontext: &'a WGContext,
               glob_pipeline: &'a PipelineState) -> Self {
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = wgcontext.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        RenderUtil {
            encoder,
            view,
            context: wgcontext,
            pipeline: glob_pipeline,
        }
    }

    /// 创建渲染中间变量
    pub fn create_render_pass(&mut self) -> wgpu::RenderPass {
        let render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &self.view,
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

    pub fn clear_frame(&mut self, color: RGBA) {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: color.0 as f64,
                        g: color.1 as f64,
                        b: color.2 as f64,
                        a: color.3 as f64,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
    }
}
