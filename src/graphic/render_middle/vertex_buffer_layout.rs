use wgpu::*;

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

    fn create_render_pipeline(device: &Device,
                              fill_topology: PrimitiveTopology,
    ) -> RenderPipeline {
        let shader = Self::set_shader(device);
        let render_pipeline = device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&Self::set_pipeline_layout(device)),
                vertex: VertexState {
                    module: &shader.vs_module,
                    entry_point: "main",
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
                    module: &shader.fs_module,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        write_mask: wgpu::ColorWrite::ALL,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    }],
                }),
            });
        return render_pipeline;
    }
}