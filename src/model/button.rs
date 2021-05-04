use wgpu::{Device, RenderPipeline};

use crate::graphic::render_type::buffer_state::VertexBuffer;
use crate::graphic::render_type::pipeline_state::PipelineState;
use crate::graphic::render_type::render_function::RenderGraph;
use crate::graphic::render_type::texture_state::TextureState;
use crate::graphic::shape::point::Rectangle;
use crate::graphic::shape::round_rect::RectState;
use crate::graphic::shape::text::TextState;
use crate::graphic::shape::triangle::RGBA;

/// 按钮属性：矩形，背景颜色，聚焦颜色，文字颜色，文本内容
pub struct Button<'a> {
    size: &'a Rectangle,
    font_color: RGBA,
    background_color: RGBA,
    border_color: RGBA,
    hover_color: RGBA,
    text: &'a str,
}

impl<'a> Button<'a> {
    pub fn new(rect: &'a Rectangle, font_color: RGBA, background_color: RGBA, border_color: RGBA, hover_color: RGBA, text: &'a str) -> Self {
        Self {
            size: rect,
            font_color,
            background_color,
            border_color,
            hover_color,
            text,
        }
    }
    pub fn default(rect: &'a Rectangle, text: &'a str) -> Self {
        let font_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let background_color = RGBA([0.8, 0.8, 0.8, 1.0]);
        let border_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let hover_color = RGBA([0.5, 0.0, 0.5, 0.5]);
        log::info!("create the Button obj");
        Self::new(rect, font_color, background_color, border_color, hover_color, text)
    }

    pub fn to_graph(&self, device: &Device, sc_desc: &wgpu::SwapChainDescriptor, queue: &wgpu::Queue) -> RenderGraph {
        let vertex_buffer = VertexBuffer::create_tex_vertex_buf(device, sc_desc, self.size);
        let shape_vertex_buffer = VertexBuffer::create_background_buf(device, sc_desc, self.size, self.background_color);
        let hover_vertex_buffer = VertexBuffer::create_background_buf(device, sc_desc, self.size, self.hover_color);
        let boder_vertex_buffer = VertexBuffer::create_border_buf(device, sc_desc, self.size, self.border_color);
        let texture_state = TextureState::create_text_texture(device, queue, self.text);

        RenderGraph {
            vertex_buffer,
            back_buffer: shape_vertex_buffer,
            hover_buffer: Some(hover_vertex_buffer),
            border_buffer: boder_vertex_buffer,
            context_buffer: texture_state,
        }
    }
    // pub fn render(&'a self, pipeline_state: &'a PipelineState, device: &Device, queue: &wgpu::Queue, sc_desc: &wgpu::SwapChainDescriptor, encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView) {
    //     let rect_state = RectState::new(device, sc_desc, &self.size, RGBA([1.0, 1.0, 1.0, 1.0]));
    //     let words = TextState::new(device, sc_desc, &self.size);
    //     let text_state = TextureState::create_text_texture(device, queue, "text");
    //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: None,
    //         color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
    //             attachment: &target,
    //             resolve_target: None,
    //             ops: wgpu::Operations {
    //                 load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
    //                 store: true,
    //             },
    //         }],
    //         depth_stencil_attachment: None,
    //     });
    //     // rect_state.render(&pipeline_state, &mut render_pass);
    //     //
    //     // words.render(&pipeline_state, &text_state, &mut render_pass);
    // }
}
