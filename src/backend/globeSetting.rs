use std::iter;

const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use wgpu::*;
use crate::backend::shape::*;
use crate::backend::mywgpu;
use crate::backend::shader::Shader;
use crate::backend::bufferState::*;
use crate::backend::render::*;
const INDICES: &[u16] = &[0, 2, 1, 3];

pub struct GlobeState {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    // NEW!
    use_complex: bool,
}

impl GlobeState {
    pub async fn new(window: &Window) -> Self {
        // ---
        let size = window.inner_size();
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::DX11);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&mywgpu::description::create_adapter_descriptor(&surface))
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &mywgpu::description::create_device_descriptor(),
                None, // Trace path
            )
            .await
            .unwrap();

        let sc_desc = mywgpu::description::be_rgba_swap_chain_descriptor(size.width, size.height);
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let use_complex = false;
        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            use_complex,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
                ..
            } => {
                self.use_complex = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self, rect: &Rectangle) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        // 纹理渲染配置：纹理着色器，文字着色器，矩形着色器。
        let texture_state = TextureState::create_texture_group(&self);
        let font_shader = Shader::create_font_shader(&self);
        let shape_shader = Shader::create_shape_shader(&self);

        // 固定渲染管道配置：纹理管道，矩形管道，边框管道。
        // 全局设置
        let render_pipeline =
            PipelineState::create_pipeline_state(&self, &font_shader, RenderType::Texture(&texture_state.texture_bind_group_layout));
        let shape_pipeline =
            PipelineState::create_pipeline_state(&self, &shape_shader, RenderType::Shape);
        let border_pipeline =
            PipelineState::create_pipeline_state(&self, &shape_shader, RenderType::Border);
        // 顶点缓冲配置：纹理顶点缓冲，矩形纹理缓冲，边框矩形缓冲。
        // 自定义设置
        let vertex_buffer = VertexBuffer::create_tex_vertex_buf(&self, &rect);
        let shape_vertex_buffer = VertexBuffer::create_shape_vertex_buf(&self, &rect);
        let boder_vertex_buffer = VertexBuffer::create_border_vertex_buf(&self, &rect);

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_shape(&mut render_pass, &shape_pipeline, &shape_vertex_buffer);
        render_shape(&mut render_pass, &border_pipeline, &boder_vertex_buffer);
        render_texture(&mut render_pass,&texture_state,&render_pipeline,&vertex_buffer);
        if self.use_complex {
            render_shape(&mut render_pass, &shape_pipeline, &shape_vertex_buffer);
        }
        drop(render_pass);
        self.queue.submit(iter::once(encoder.finish()));

        Ok(())
    }

    pub fn render_a_button() {}
}


/// 定义三种渲染类型：纹理，全填充图形，线框图形
/// 主要用在创建渲染管道方法中定义渲染管道[`create_pipeline_state`]
pub enum RenderType<'a> {
    Texture(&'a BindGroupLayout),
    Shape,
    Border,
}

/// 渲染管道状态元结构体
pub struct PipelineState;

impl<'a> PipelineState {
    /// 创建渲染管道
    /// 参数：全局状态，着色器，渲染类型
    pub fn create_pipeline_state(globe_state: &'a GlobeState, shader: &'a Shader, render_type: RenderType) -> RenderPipeline {
        let render_pipeline_layout;
        let vertex_desc;
        let fill_pology;
        match render_type {
            RenderType::Texture(texture_bind_group_layout) => {
                render_pipeline_layout = globe_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[texture_bind_group_layout],
                    push_constant_ranges: &[],
                });
                vertex_desc = [TexturePoint::desc()];
                fill_pology = wgpu::PrimitiveTopology::TriangleStrip;
            }
            RenderType::Shape => {
                render_pipeline_layout = globe_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
                vertex_desc = [BufferPoint::desc()];
                fill_pology = wgpu::PrimitiveTopology::TriangleStrip;
            }
            RenderType::Border => {
                render_pipeline_layout = globe_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
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
        let render_pipeline = globe_state.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex_stage: mywgpu::description::create_shader_descriptor(&shader.vs_module),
                fragment_stage: Some(mywgpu::description::create_shader_descriptor(&shader.fs_module)),
                rasterization_state: Some(mywgpu::description::create_rasterization_state_descriptor()),
                primitive_topology: fill_pology,
                color_states: &[mywgpu::description::be_blend_color_state_descriptor(globe_state.sc_desc.format)],
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

