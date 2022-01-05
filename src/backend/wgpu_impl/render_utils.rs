use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

use crate::backend::wgpu_impl::*;
use crate::device::GPUContext;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::Style;

/// 渲染工具封装结构体
/// 作用：省事
#[derive(Debug)]
pub struct RenderUtil<'a> {
    pub encoder: CommandEncoder,
    pub view: TextureView,
    pub context: &'a mut GPUContext,
    pub g_texture: GTexture,
}

impl<'a> RenderUtil<'a> {
    pub fn new(target_view: &SurfaceTexture,
               gpu_context: &'a mut GPUContext) -> Self {
        let view = target_view
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = gpu_context.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        let g_texture = GTexture::new(&gpu_context.device,
                                      Point::new(40, 40), wgpu::TextureFormat::R8Unorm);
        RenderUtil {
            encoder,
            view,
            context: gpu_context,
            g_texture,
        }
    }
}

impl PaintBrush for RenderUtil<'_> {
    fn clear_frame(&mut self, color: RGBA) {
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

    fn draw_shape(&mut self, shape: &Box<dyn ShapeGraph>, shape_style: Style) {
        let shape_buffer = shape.to_buffer(self.context, shape_style);
        shape_buffer.render(self, shape.get_type());
    }

    fn draw_text(&mut self, font_map: &mut GCharMap, text_rect: &Rectangle, text: &str, text_color: RGBA) {
        let mut x = text_rect.position.x + 8.;
        let scale = 10. / (font_map.scale / 2.5);
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

    fn draw_image(&mut self, image_rect: &Rectangle, image: ImageRaw) {}
}