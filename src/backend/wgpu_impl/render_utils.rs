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
        create_render_pass(&mut self.encoder, &self.view, RenderModel::Clear(color));
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
            let c_font = font_map.character_texture(c, self);
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

    fn draw_image(&mut self, image_rect: &Rectangle, image: ImageRaw) {
        let image_buffer =
            self.g_texture
                .create_bind_group(&self.context.device, &self.context.queue, image);
        let image_vertex = TextureVertex::new(&self.context, &image_rect, ALPHA);
        image_vertex.render(self, Some(&image_buffer))
    }
}
