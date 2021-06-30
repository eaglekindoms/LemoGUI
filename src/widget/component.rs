use wgpu::*;

use crate::device::display_window::WGContext;
use crate::graphic::base::image2d::TextureVertex;
use crate::graphic::base::rectangle::*;
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::RenderGraph;
use crate::graphic::render_middle::texture_buffer::TextureBuffer;
use crate::graphic::render_middle::vertex_buffer_layout::VertexInterface;
use crate::graphic::style::*;
use crate::widget::listener::Listener;

/// 组件属性：矩形，背景颜色，聚焦颜色，文字颜色，文本内容
#[derive(Debug, Default)]
pub struct Component<L: Listener + ?Sized> {
    size: Rectangle,
    text: String,
    render_buffer: Option<RenderGraph>,
    listener: Option<Box<L>>,
    style: Style,
}

pub trait ComponentModel {
    fn set_index(&mut self, index: usize);
    fn get_index(&self) -> Option<usize>;
    fn to_graph(&mut self, wgcontext: &WGContext) -> &RenderGraph;
    fn get_style(&self) -> &Style;
    fn draw(&mut self, wgcontext: &WGContext, encoder: &mut CommandEncoder, target: &TextureView, glob_pipeline: &PipelineState) {
        let render_buffer = self.to_graph(wgcontext);

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        render_buffer.draw(&mut render_pass, &glob_pipeline);
    }
}

impl<'a> Component<dyn Listener> {
    pub fn new<S: Into<String>>(rect: Rectangle, style: Style,
                                text: S, listener: Option<Box<dyn Listener>>) -> Self {
        Self {
            size: rect,
            text: text.into(),
            render_buffer: None,
            listener,
            style,
        }
    }

    pub fn to_graph(&mut self, display_window: &WGContext) -> &RenderGraph {
        match self.render_buffer.as_mut() {
            Some(_) => {}
            None => {
                let vertex_buffer =
                    TextureVertex::from_shape_to_vector
                        (&display_window.device, &display_window.sc_desc, &self.size);
                let back_buffer =
                    RectVertex::from_shape_to_vector
                        (&display_window.device, &display_window.sc_desc, &self.size, &self.style);

                let font_buffer =
                    TextureBuffer::create_font_image
                        (&display_window.device,
                         &display_window.queue, self.style.get_font_color(), self.text.as_str());

                self.render_buffer = Some(RenderGraph {
                    vertex_buffer,
                    back_buffer,
                    context_buffer: font_buffer,
                });
            }
        }
        self.render_buffer.as_ref().unwrap()
    }

    pub fn get_style(&self) -> &Style {
        &self.style
    }
}