use wgpu::{CommandEncoder, TextureView};

/// 渲染工具封装结构体
/// 作用：省事
#[derive(Debug)]
pub struct RenderUtil {
    pub encoder: CommandEncoder,
    pub view: TextureView,
}

impl RenderUtil {
    /// 创建渲染中间变量
    pub fn create_render_pass<'a>(&'a mut self) -> wgpu::RenderPass<'a> {
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
}