use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

use crate::backend::wgpu_impl::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::Style;

/// 渲染工具封装结构体
/// 基于wgpu实现渲染API定义的基本渲染方法
#[derive(Debug)]
pub struct RenderUtil<'a> {
    /// wgpu提供的gpu命令编码器，用于发送渲染命令
    pub encoder: CommandEncoder,
    /// 目标渲染区域
    pub view: TextureView,
    /// 图形渲染上下文
    pub context: &'a mut WGPUContext,
    /// 纹理配置上下文
    pub g_texture: GTexture,
    /// 彩色图像纹理
    pub g_image: GTexture,
}

impl<'a> RenderUtil<'a> {
    /// 创建渲染工具
    /// 参数：目标渲染区域，图形渲染上下文
    pub fn new(target_view: &SurfaceTexture, gpu_context: &'a mut WGPUContext) -> Self {
        let view = target_view
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = gpu_context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        // 默认创建40x40纹理的配置，用于文字渲染
        let g_texture = GTexture::new(
            &gpu_context.device,
            Point::new(40, 40),
            wgpu::TextureFormat::R8Unorm,
        );
        let g_image = GTexture::new(
            &gpu_context.device,
            Point::new(40, 40),
            wgpu::TextureFormat::Rgba8Unorm,
        );
        RenderUtil {
            encoder,
            view,
            context: gpu_context,
            g_texture,
            g_image,
        }
    }
}

impl PaintBrush for RenderUtil<'_> {
    fn clear_frame(&mut self, color: RGBA) {
        vertex_buffer::create_render_pass(&mut self.encoder, &self.view, RenderModel::Clear(color));
    }

    fn draw_shape(&mut self, shape: &Box<dyn ShapeGraph>, shape_style: Style) {
        let shape_buffer = shape.to_buffer(self.context, shape_style);
        shape_buffer.render(self, None);
    }

    fn draw_text(
        &mut self,
        font_map: &mut GCharMap,
        text_rect: &Rectangle,
        text: &str,
        text_color: RGBA,
    ) {
        let mut x = text_rect.position.x + 8.;
        let scale = 10. / (font_map.scale / 2.5);
        for c in text.chars() {
            let c_font = font_map.character_texture(
                c,
                &mut self.g_texture,
                &self.context.device,
                &self.context.queue,
            );
            let c_buffer = c_font.texture.as_ref().unwrap();
            let c_x = x;
            let c_y = text_rect.position.y;
            let scale_width = c_buffer.width as f32 * scale;
            let c_rect = Rectangle::new(c_x, c_y, scale_width as u32, c_buffer.height);
            x = x + scale_width;
            let c_vertex = TextureVertex::new(&self.context, &c_rect, text_color);

            c_vertex.render(self, Some(&c_buffer));
        }
    }

    fn draw_styled_text(
        &mut self,
        font_map: &mut GCharMap,
        origin: Point<f32>,
        text: &str,
        style: TextStyle,
    ) -> f32 {
        let start_x = origin.x;
        let mut x = origin.x;
        let bucket = quantize_size(style.size);
        let skew = if style.italic {
            bucket as f32 * 0.22
        } else {
            0.0
        };
        for c in text.chars() {
            if c == '\n' || c == '\r' {
                continue;
            }
            let c_font = font_map.character_texture_at(
                c,
                style.size,
                style.font,
                &mut self.g_texture,
                &self.context.device,
                &self.context.queue,
            );
            let c_buffer = c_font.texture.as_ref().unwrap();
            let w = c_buffer.width;
            let h = c_buffer.height;
            let c_rect = Rectangle::new(x, origin.y, w, h);
            let vertex =
                TextureVertex::new_with_skew(&self.context, &c_rect, style.color, skew);
            vertex.render(self, Some(c_buffer));
            if style.bold {
                let bold_rect = Rectangle::new(x + 1.0, origin.y, w, h);
                let bold_vertex =
                    TextureVertex::new_with_skew(&self.context, &bold_rect, style.color, skew);
                bold_vertex.render(self, Some(c_buffer));
            }
            x += w as f32;
            if style.bold {
                x += 1.0;
            }
        }
        let width = x - start_x;
        if style.underline && width > 1.0 {
            let line_y = origin.y + bucket as f32 * 0.85;
            let line = Rectangle::new(start_x, line_y, width as u32, 2);
            let shape: Box<dyn ShapeGraph> = Box::new(line);
            self.draw_shape(
                &shape,
                Style::default().back_color(style.color).no_border(),
            );
        }
        width
    }

    fn draw_image(&mut self, image_rect: &Rectangle, image: ImageRaw) {
        let image_buffer =
            self.g_image
                .create_bind_group(&self.context.device, &self.context.queue, image);
        let image_vertex = ColorImageVertex::new(&self.context, image_rect, WHITE);
        image_vertex.render(self, Some(&image_buffer))
    }
}
