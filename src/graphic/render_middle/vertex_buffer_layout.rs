use wgpu::{Device, PipelineLayout, PrimitiveTopology};

use crate::graphic::base::color::RGBA;
use crate::graphic::base::rectangle::Rectangle;
use crate::graphic::render_middle::pipeline_state::Shader;

pub trait VertexInterface: Sized {
    fn set_vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a>;
    fn set_shader(device: &Device) -> Shader;
    fn set_pipeline_layout(device: &Device) -> PipelineLayout {
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        return render_pipeline_layout;
    }
    fn from_shape_to_vector(rect: &Rectangle, sc_desc: &wgpu::SwapChainDescriptor, test_color: RGBA) -> Vec<Self>;
}