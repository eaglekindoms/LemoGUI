use wgpu::*;

/// wgpu图形顶点布局trait
/// 作用：定义顶点布局接口
pub trait VertexLayout: Sized {
    /// 设置图形顶点缓存布局
    fn set_vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a>;
    /// 设置图元渲染器
    fn get_shader(device: &Device) -> ShaderModule;
    /// 设置渲染管线布局
    fn set_pipeline_layout(device: &Device) -> PipelineLayout {
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        return render_pipeline_layout;
    }
    /// 创建渲染管线
    fn create_render_pipeline(device: &Device,
                              fill_topology: PrimitiveTopology,
    ) -> RenderPipeline {
        let shader = Self::get_shader(device);
        let render_pipeline = device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&Self::set_pipeline_layout(device)),
                vertex: VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Self::set_vertex_desc()],
                },
                primitive: wgpu::PrimitiveState {
                    topology: fill_topology,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        write_mask: wgpu::ColorWrites::ALL,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    }],
                }),
                multiview: None,
            });
        return render_pipeline;
    }
}