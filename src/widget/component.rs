use std::option::Option::Some;

use wgpu::RenderPipeline;

use crate::device::display_window::WGContext;
use crate::graphic::base::color::RGBA;
use crate::graphic::base::image2d::{TextureBuffer, TextureVertex};
use crate::graphic::base::point2d::PointVertex;
use crate::graphic::base::rectangle::Rectangle;
use crate::graphic::render_middle::pipeline_state::PipelineState;
use crate::graphic::render_middle::render_function::RenderGraph;
use crate::graphic::render_middle::vertex_buffer::VertexBuffer;
use crate::widget::listener::Listener;

/// 组件属性：矩形，背景颜色，聚焦颜色，文字颜色，文本内容
#[derive(Debug, Default, Clone)]
pub struct Component<'a, L: Listener + ?Sized> {
    size: Rectangle,
    font_color: RGBA,
    background_color: RGBA,
    border_color: RGBA,
    hover_color: RGBA,
    text: &'a str,
    listener: Option<Box<L>>,
}

pub trait ComponentModel {
    fn set_index(&mut self, index: usize);
    fn get_index(&self) -> Option<usize>;
    fn to_graph(&self, wgcontext: &WGContext) -> RenderGraph;
}

type CompNode<'a> = Option<Box<&'a CompTreeNode<'a, dyn Listener>>>;

#[derive(Clone)]
pub struct CompTreeNode<'a, L: Listener + ?Sized> {
    context: Component<'a, L>,
    parent: CompNode<'a>,
    next: CompNode<'a>,
    child: CompNode<'a>,
}

pub struct CompTree<'a, L: Listener + ?Sized> {
    root: Option<CompTreeNode<'a, L>>,
    size: i32,
}

impl<'a> Component<'a, dyn Listener> {
    pub fn new(rect: Rectangle, font_color: RGBA, background_color: RGBA,
               border_color: RGBA, hover_color: RGBA,
               text: &'a str, listener: Box<dyn Listener>) -> Self {
        Self {
            size: rect,
            font_color,
            background_color,
            border_color,
            hover_color,
            text,
            listener: Option::from(listener),
        }
    }

    pub fn default(rect: Rectangle, font_color: RGBA, background_color: RGBA, border_color: RGBA, hover_color: RGBA, text: &'a str) -> Self {
        Self {
            size: rect,
            font_color,
            background_color,
            border_color,
            hover_color,
            text,
            listener: None,
        }
    }

    pub fn to_graph(&self, display_window: &WGContext) -> RenderGraph {
        let vertex_buffer = VertexBuffer::create_vertex_buf::<TextureVertex>(&display_window.device, &display_window.sc_desc, &self.size, &[0, 2, 1, 3], RGBA::default());
        let shape_vertex_buffer = VertexBuffer::create_vertex_buf::<PointVertex>(&display_window.device, &display_window.sc_desc, &self.size, &[0, 2, 1, 3], self.background_color);
        let hover_vertex_buffer = VertexBuffer::create_vertex_buf::<PointVertex>(&display_window.device, &display_window.sc_desc, &self.size, &[0, 2, 1, 3], self.hover_color);
        let boder_vertex_buffer = VertexBuffer::create_vertex_buf::<PointVertex>(&display_window.device, &display_window.sc_desc, &self.size, &[0, 1, 3, 2, 0], self.border_color);
        let font_buffer = TextureBuffer::create_font_image(&display_window.device, &display_window.queue, self.font_color, self.text);

        // let round_vertex_buffer = RectVertex
        RenderGraph {
            vertex_buffer,
            back_buffer: shape_vertex_buffer,
            hover_buffer: Some(hover_vertex_buffer),
            border_buffer: boder_vertex_buffer,
            context_buffer: font_buffer,
        }
    }
}

impl<'a> CompTreeNode<'a, dyn Listener> {
    pub fn new(component: Component<'a, dyn Listener>) -> Self {
        CompTreeNode {
            context: component,
            parent: None,
            next: None,
            child: None,
        }
    }
    pub fn insert_child(&'a mut self, mut treeNode: CompTreeNode<'a, dyn Listener>) {
        treeNode.parent = Some(Box::new(self));
        let mut child = self.child.as_ref();
        if child.is_none() {
            child = Some(&Box::new(&treeNode));
        } else {
            while child.is_some() {
                let mut current_node = child.unwrap().next.as_ref();
                if current_node.is_some() {
                    let mut future_node = current_node.unwrap().next.as_ref();
                    if future_node.is_none() {
                        future_node = Some(&Box::new(&treeNode));
                        break;
                    } else {
                        child = future_node;
                    }
                } else {
                    current_node = Some(&Box::new(&treeNode));
                    break;
                }
            }
        }
    }
}

impl<'a> CompTree<'a, dyn Listener> {
    pub fn new() -> Self {
        CompTree {
            root: None,
            size: 0,
        }
    }
}