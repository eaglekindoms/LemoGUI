use crate::device::display_window::WGContext;
use crate::graphic::base::image_vertex::TextureVertex;
use crate::graphic::base::rect_vertex::*;
use crate::graphic::base::shape::Rectangle;
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::{RenderGraph, RenderUtil};
use crate::graphic::render_middle::texture_buffer::TextureBuffer;
use crate::graphic::style::*;
use crate::widget::listener::Listener;

/// 公共组件结构体
#[derive(Debug, Default)]
pub struct Component {
    /// 组件尺寸
    size: Rectangle,
    /// 图形缓存
    render_buffer: Option<RenderGraph>,
    /// 组件样式
    style: Style,
    /// 是否重绘
    is_redraw: bool,
}

/// 组件模型trait
/// 作用：定义组件必须的公共方法接口
pub trait ComponentModel: Listener {
    #[deprecated]
    fn to_graph(&mut self, _wgcontext: &WGContext) -> Option<&RenderGraph> { None }

    /// 组件绘制方法实现
    fn draw(&mut self, wgcontext: &WGContext, render_utils: &mut RenderUtil, glob_pipeline: &PipelineState) {
        match self.to_graph(wgcontext) {
            Some(render_buffer) => {
                render_buffer.draw(render_utils, &glob_pipeline);
            }
            None => {}
        }
    }
}


impl<'a> Component {
    pub fn new(rect: Rectangle, style: Style) -> Self {
        Self {
            size: rect,
            render_buffer: None,
            style,
            is_redraw: false,
        }
    }
    pub fn is_redraw(&self) -> bool {
        self.is_redraw
    }
    pub fn set_is_redraw(&mut self, is_redraw: bool) {
        self.is_redraw = is_redraw;
    }
    #[deprecated]
    pub fn to_graph(&mut self, text: &String, display_window: &WGContext) -> &RenderGraph {
        if self.is_redraw {
            self.render_buffer = Some(self.convert_graph(text, display_window));
            self.is_redraw = false;
        } else {
            match self.render_buffer.as_mut() {
                Some(_) => {}
                None => {
                    self.render_buffer = Some(self.convert_graph(text, display_window));
                }
            }
        }

        self.render_buffer.as_ref().unwrap()
    }
    pub fn get_style(&self) -> &Style {
        &self.style
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }
    #[deprecated]
    fn convert_graph(&self, text: &String, display_window: &WGContext) -> RenderGraph {
        let vertex_buffer =
            TextureVertex::from_shape_to_vector
                (&display_window.device, &display_window.sc_desc, &self.size);
        let back_buffer =
            RectVertex::from_shape_to_vector
                (&display_window.device, &display_window.sc_desc, &self.size, &self.style);

        let font_buffer =
            TextureBuffer::create_font_image
                (&display_window.device,
                 &display_window.queue, self.style.get_font_color(), text.as_str());
        log::info!("new render buffer");
        RenderGraph {
            vertex_buffer,
            back_buffer,
            context_buffer: font_buffer,
        }
    }
}
