use wgpu::*;

use crate::graphic::base::image2d::TextureVertex;
use crate::graphic::base::point2d::PointVertex;
use crate::graphic::base::rectangle::RectVertex;
use crate::graphic::render_middle::vertex_buffer_layout::VertexInterface;

/// 渲染管道状态元结构体
pub struct PipelineState {
    pub texture_pipeline: RenderPipeline,
    pub shape_pipeline: RenderPipeline,
    pub border_pipeline: RenderPipeline,
    pub round_shape_pipeline: RenderPipeline,
}

pub struct Shader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl<'a> PipelineState {
    pub fn create_glob_pipeline(device: &Device) -> Self {
        // 固定渲染管道配置：纹理管道，矩形管道，边框管道。
        // 全局设置
        use wgpu::PrimitiveTopology::{TriangleStrip, LineStrip};
        let texture_pipeline =
            PipelineState::create_pipeline_state::<TextureVertex>(device, TriangleStrip);
        let shape_pipeline =
            PipelineState::create_pipeline_state::<RectVertex>(device, TriangleStrip);
        let border_pipeline =
            PipelineState::create_pipeline_state::<PointVertex>(device, LineStrip);
        let round_shape_pipeline =
            PipelineState::create_pipeline_state::<RectVertex>(device, TriangleStrip);

        log::info!("create the PipelineState obj");
        Self {
            texture_pipeline,
            shape_pipeline,
            border_pipeline,
            round_shape_pipeline,
        }
    }
    /// 创建渲染管道
    /// 参数：全局状态，着色器，渲染类型
    pub fn create_pipeline_state<V>(device: &Device, fill_topology: PrimitiveTopology) -> RenderPipeline
        where V: VertexInterface {
        let shader = V::set_shader(device);
        let render_pipeline_layout = V::set_pipeline_layout(device);
        let vertex_desc = [V::set_vertex_desc()];
        // 作用：绑定着色器，图形填充
        let render_pipeline = create_render_pipeline(device
                                                     , shader, render_pipeline_layout, vertex_desc, fill_topology);
        return render_pipeline;
    }
}

pub fn create_render_pipeline(device: &Device, shader: Shader,
                              render_pipeline_layout: PipelineLayout,
                              vertex_desc: [VertexBufferLayout; 1],
                              fill_topology: PrimitiveTopology,
) -> RenderPipeline {
    let render_pipeline = device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader.vs_module,
                entry_point: "main",
                buffers: &vertex_desc,
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