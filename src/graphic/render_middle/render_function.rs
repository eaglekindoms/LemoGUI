use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

use crate::device::WGContext;
use crate::graphic::base::{GCharMap, ImageRaw, Point, Rectangle, RGBA, ShapeGraph};
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
    pub fn new(target_view: &SurfaceTexture,
               wgcontext: &'a mut WGContext,
               glob_pipeline: &'a PipelineState) -> Self {
        let view = target_view
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

    pub fn draw_text(&mut self, font_map: &mut GCharMap<'static>, text_rect: &Rectangle, text: &str, mut text_color: RGBA) {
        let mut x = text_rect.position.x;
        let scale = text_rect.width as f32 / (text.len() as f32 * font_map.scale / 2.5);
        for c in text.chars() {
            let c_font = font_map.character_texture(c, &mut self.g_texture,
                                                    &self.context.device, &self.context.queue);
            let c_buffer = c_font.texture.as_ref().unwrap();
            let c_x = x;
            let c_y = text_rect.position.y;
            let scale_width = c_buffer.width as f32 * scale;
            let c_rect =
                Rectangle::new(c_x, c_y, scale_width as u32, c_buffer.height);
            x = x + scale_width;
            let c_vertex =
                TextureVertex::new(&self.context.device,
                                   &self.context.get_surface_size(), &c_rect, text_color);

            c_vertex.render_t(self, &c_buffer);
        }
    }

    pub fn draw_image(&mut self, image_rect: &Rectangle, image: ImageRaw) {}
}
