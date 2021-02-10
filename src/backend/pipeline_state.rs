use wgpu::{BindGroupLayout, RenderPipeline};
use crate::backend::global_setting::GlobalState;
use crate::backend::buffer_state::TextureState;
use crate::backend::shader::Shader;
use crate::backend::mywgpu;
use crate::backend::shape::{TexturePoint, BufferPoint};

/// 定义三种渲染类型：纹理，全填充图形，线框图形
/// 主要用在创建渲染管道方法中定义渲染管道[`create_pipeline_state`]
pub enum RenderType<'a> {
    Texture(&'a BindGroupLayout),
    Shape,
    Border,
}

/// 渲染管道状态元结构体
pub struct PipelineState {
    pub render_pipeline: RenderPipeline,
    pub shape_pipeline: RenderPipeline,
    pub border_pipeline: RenderPipeline,
}

impl<'a> PipelineState {
    pub fn create_glob_pipeline(global_state: &'a GlobalState, texture_state: &'a TextureState) -> Self {
        // 纹理渲染配置：纹理着色器，文字着色器，矩形着色器。
        let font_shader = Shader::create_font_shader(global_state);
        let shape_shader = Shader::create_shape_shader(global_state);

        // 固定渲染管道配置：纹理管道，矩形管道，边框管道。
        // 全局设置
        let render_pipeline =
            PipelineState::create_pipeline_state(global_state, &font_shader, RenderType::Texture(&texture_state.texture_bind_group_layout));
        let shape_pipeline =
            PipelineState::create_pipeline_state(global_state, &shape_shader, RenderType::Shape);
        let border_pipeline =
            PipelineState::create_pipeline_state(global_state, &shape_shader, RenderType::Border);
        Self {
            render_pipeline,
            shape_pipeline,
            border_pipeline,
        }
    }
    /// 创建渲染管道
    /// 参数：全局状态，着色器，渲染类型
    pub fn create_pipeline_state(global_state: &'a GlobalState, shader: &'a Shader, render_type: RenderType) -> RenderPipeline {
        let render_pipeline_layout;
        let vertex_desc;
        let fill_pology;
        match render_type {
            RenderType::Texture(texture_bind_group_layout) => {
                render_pipeline_layout = global_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[texture_bind_group_layout],
                    push_constant_ranges: &[],
                });
                vertex_desc = [TexturePoint::desc()];
                fill_pology = wgpu::PrimitiveTopology::TriangleStrip;
            }
            RenderType::Shape => {
                render_pipeline_layout = global_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
                vertex_desc = [BufferPoint::desc()];
                fill_pology = wgpu::PrimitiveTopology::TriangleStrip;
            }
            RenderType::Border => {
                render_pipeline_layout = global_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
                vertex_desc = [BufferPoint::desc()];
                fill_pology = wgpu::PrimitiveTopology::LineStrip;
            }
            _ => panic!(),
        };

        // 作用：绑定着色器，图形填充
        let render_pipeline = global_state.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex_stage: mywgpu::description::create_shader_descriptor(&shader.vs_module),
                fragment_stage: Some(mywgpu::description::create_shader_descriptor(&shader.fs_module)),
                rasterization_state: Some(mywgpu::description::create_rasterization_state_descriptor()),
                primitive_topology: fill_pology,
                color_states: &[mywgpu::description::be_blend_color_state_descriptor(global_state.sc_desc.format)],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &vertex_desc,
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });
        return render_pipeline;
    }
}