use wgpu::PrimitiveTopology::*;
use wgpu::*;

use crate::backend::wgpu_impl::ShapeType;

/// wgpu图形顶点布局trait
/// 作用：定义顶点布局接口
pub trait VertexLayout: Sized {
    /// 设置图形顶点缓存布局
    fn set_vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a>;
    /// 获取顶点布局类型
    fn get_shape_type() -> ShapeType;
    /// 设置图元渲染器
    fn get_shader(device: &Device) -> ShaderModule;
    /// 设置渲染管线布局
    fn set_pipeline_layout(device: &Device) -> PipelineLayout {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });
        return render_pipeline_layout;
    }
    /// 创建渲染管线
    fn create_render_pipeline(device: &Device) -> RenderPipeline {
        let shader = Self::get_shader(device);
        let fill_topology = get_fill_topology_by_type(Self::get_shape_type());
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&Self::set_pipeline_layout(device)),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Self::set_vertex_desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: fill_topology,
                strip_index_format: match fill_topology {
                    wgpu::PrimitiveTopology::TriangleStrip | wgpu::PrimitiveTopology::LineStrip => {
                        Some(wgpu::IndexFormat::Uint16)
                    }
                    _ => None,
                },
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    write_mask: wgpu::ColorWrites::ALL,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        return render_pipeline;
    }
}

/// 由形状类型获取指定的索引模式
fn get_fill_topology_by_type(shape_type: ShapeType) -> PrimitiveTopology {
    match shape_type {
        ShapeType::TEXTURE => TriangleStrip,
        ShapeType::ROUND => TriangleStrip,
        ShapeType::BORDER => LineStrip,
        ShapeType::POINT => TriangleList,
        ShapeType::Circle => TriangleStrip,
    }
}
