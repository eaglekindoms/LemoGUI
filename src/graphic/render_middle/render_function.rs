use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

use crate::device::WGContext;
use crate::graphic::base::{ImageRaw, Point, Rectangle, RGBA, ShapeGraph};
use crate::graphic::render_middle::{GTexture, TextureVertex};
use crate::graphic::render_middle::pipeline_state::PipelineState;

/// 渲染工具封装结构体
/// 作用：省事
#[derive(Debug)]
pub struct RenderUtil<'a> {
    pub encoder: CommandEncoder,
    pub view: TextureView,
    pub context: &'a mut WGContext,
    pub pipeline: &'a PipelineState,
    pub g_texture: GTexture,
}

impl<'a> RenderUtil<'a> {
    pub fn new(frame: &SurfaceTexture,
               wgcontext: &'a mut WGContext,
               glob_pipeline: &'a PipelineState) -> Self {
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = wgcontext.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        let g_texture = GTexture::new(&wgcontext.device,
                                      Point::new(40, 40), wgpu::TextureFormat::R8Unorm);
        RenderUtil {
            encoder,
            view,
            context: wgcontext,
            pipeline: glob_pipeline,
            g_texture,
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

    pub fn draw_rect(&mut self, rect: &Rectangle, rect_color: RGBA) {
        let rect_buffer = rect.to_buffer(self.context, rect_color);
        rect_buffer.render(self, rect.get_type());
    }
    pub fn draw_text(&mut self, text_rect: &Rectangle, text: &str, text_color: RGBA) {
        let image_vertex_buffer =
            TextureVertex::new(&self.context.device,
                               self.context.get_surface_size(), text_rect, text_color);
        let font_buffer = self.g_texture
            .fill_text(self.context, text);
        image_vertex_buffer.render_t(self, &font_buffer);
    }

    pub fn draw_image(&mut self, image_rect: &Rectangle, image: ImageRaw) {}
}
