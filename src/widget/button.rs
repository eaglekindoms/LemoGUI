use std::fmt::Debug;

use wgpu::{CommandEncoder, RenderPass, RenderPipeline, TextureView};
use winit::event::WindowEvent;

use crate::device::display_window::WGContext;
use crate::graphic::base::color::RGBA;
use crate::graphic::base::point2d::Point;
use crate::graphic::base::rectangle::Rectangle;
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::{render_shape, render_texture, RenderGraph};
use crate::widget::component::Component;
use crate::widget::component::ComponentModel;
use crate::widget::listener::Listener;

/// 按钮属性：矩形，背景颜色，聚焦颜色，文字颜色，文本内容
#[derive(Debug)]
pub struct Button<'a, L: Listener + ?Sized> {
    pub context: Component<'a, L>,
    pub index: Option<usize>,
}

impl<'a> Button<'a, dyn Listener> {
    pub fn new(rect: Rectangle, text: &'a str) -> Self {
        let font_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let background_color = RGBA([0.4, 0.8, 0.8, 1.0]);
        let border_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let hover_color = RGBA([0.5, 0.0, 0.5, 0.5]);
        log::info!("create the Button obj use new");
        let context = Component::default(rect, font_color, background_color, border_color, hover_color, text);

        Self {
            context,
            index: None,
        }
    }

    pub fn default(pos: Point, text: &'a str) -> Self {
        let font_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let background_color = RGBA([0.8, 0.8, 0.8, 1.0]);
        let border_color = RGBA([0.0, 0.0, 0.0, 1.0]);
        let hover_color = RGBA([0.5, 0.0, 0.5, 0.5]);
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32, 40);
        log::info!("create the Button obj use default");
        let context = Component::default(rect, font_color, background_color, border_color, hover_color, text);

        Self {
            context,
            index: None,
        }
    }
}

impl<'a> ComponentModel for Button<'a, dyn Listener> {
    fn set_index(&mut self, index: usize) {
        self.index = Option::from(index);
    }

    fn get_index(&self) -> Option<usize> {
        self.index
    }

    fn to_graph(&self, wgcontext: &WGContext) -> RenderGraph {
        self.context.to_graph(wgcontext)
    }

    fn draw(&self, wgcontext: &WGContext, encoder: &mut CommandEncoder, target: &TextureView, glob_pipeline: &PipelineState) {
        let render_buffer = self.to_graph(wgcontext);

        // let instance_bytes = bytemuck::cast_slice(&instances[i..end]);
        //
        // let mut instance_buffer = staging_belt.write_buffer(
        //     encoder,
        //     &self.instances,
        //     0,
        //     wgpu::BufferSize::new(instance_bytes.len() as u64).unwrap(),
        //     device,
        // );
        //
        // instance_buffer.copy_from_slice(instance_bytes);

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
        render_shape(&mut render_pass, &glob_pipeline.shape_pipeline, &render_buffer.back_buffer);
        render_shape(&mut render_pass, &glob_pipeline.border_pipeline, &render_buffer.border_buffer);
        render_texture(&mut render_pass, &render_buffer.context_buffer, &glob_pipeline.texture_pipeline, &render_buffer.vertex_buffer);
    }
}

impl<'a> Listener for Button<'a, dyn Listener> {
    fn key_listener(&mut self, event: &WindowEvent) {
        log::info!("{:?}", event);
    }
}