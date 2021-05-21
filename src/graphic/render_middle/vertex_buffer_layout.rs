use wgpu::{Device, PipelineLayout};

use crate::graphic::base::color::RGBA;
use crate::graphic::base::rectangle::Rectangle;
use crate::graphic::render_middle::shader::Shader;

pub trait VertexInterface: Sized {
    fn set_vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a>;
    fn set_shader(device: &Device) -> Shader;
    fn set_pipeline_layout(device: &Device) -> PipelineLayout;
    fn set_fill_topology() -> wgpu::PrimitiveTopology;
    fn from_shape_to_vector(rect: &Rectangle, sc_desc: &wgpu::SwapChainDescriptor, test_color: RGBA) -> Vec<Self>;
}