use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor, BlendFactor, BlendOperation, Device, RenderPipeline, VertexState};

use crate::graphic::render_type::texture_buffer::TextureBuffer;
use crate::graphic::shader::Shader;
use crate::graphic::shape::point2d::PointVertex;
use crate::graphic::shape::rectangle::RectVertex;
use crate::graphic::shape::texture2d::TextureVertex;

/// 定义三种渲染类型：纹理，全填充图形，线框图形
/// 主要用在创建渲染管道方法中定义渲染管道[`create_pipeline_state`]
pub enum RenderType {
    Texture,
    Shape,
    Border,
    RoundShape,
}

/// 渲染管道状态元结构体
pub struct PipelineState {
    pub texture_pipeline: RenderPipeline,
    pub shape_pipeline: RenderPipeline,
    pub border_pipeline: RenderPipeline,
    pub round_shape_pipeline: RenderPipeline,
}

impl<'a> PipelineState {
    pub fn create_glob_pipeline(device: &Device) -> Self {
        // 纹理渲染配置：纹理着色器，文字着色器，矩形着色器。
        let font_shader = Shader::create_font_shader(device);
        let shape_shader = Shader::create_shape_shader(device);
        let round_shader = Shader::create_round_shape_shader(device);

        // 固定渲染管道配置：纹理管道，矩形管道，边框管道。
        // 全局设置
        let texture_pipeline =
            PipelineState::create_pipeline_state(device, &font_shader, RenderType::Texture);
        let shape_pipeline =
            PipelineState::create_pipeline_state(device, &shape_shader, RenderType::Shape);
        let border_pipeline =
            PipelineState::create_pipeline_state(device, &shape_shader, RenderType::Border);
        let round_shape_pipeline =
            PipelineState::create_pipeline_state(device, &round_shader, RenderType::RoundShape);

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
    pub fn create_pipeline_state(device: &Device, shader: &'a Shader, render_type: RenderType) -> RenderPipeline {
        let render_pipeline_layout;
        let vertex_desc;
        let fill_pology;
        let texture_bind_group_layout = device.create_bind_group_layout(
            &Self::create_bind_group_layout_descriptor()
        );
        match render_type {
            RenderType::Texture => {
                render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&texture_bind_group_layout],
                    push_constant_ranges: &[],
                });
                vertex_desc = [TextureVertex::desc()];
                fill_pology = wgpu::PrimitiveTopology::TriangleStrip;
            }
            RenderType::Shape => {
                render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
                vertex_desc = [PointVertex::desc()];
                fill_pology = wgpu::PrimitiveTopology::TriangleStrip;
            }
            RenderType::Border => {
                render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
                vertex_desc = [PointVertex::desc()];
                fill_pology = wgpu::PrimitiveTopology::LineStrip;
            }
            RenderType::RoundShape => {
                render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
                vertex_desc = [RectVertex::desc()];
                fill_pology = wgpu::PrimitiveTopology::TriangleStrip;
            }
            _ => panic!(),
        };

        // 作用：绑定着色器，图形填充
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
                    topology: fill_pology,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::None,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader.fs_module,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        color_blend: wgpu::BlendState {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha_blend: wgpu::BlendState {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                }),
            });
        return render_pipeline;
    }
    fn create_bind_group_layout_descriptor() -> BindGroupLayoutDescriptor<'a> {
        BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        }
    }
}