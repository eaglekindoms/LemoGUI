use std::iter;
use ab_glyph::{point, Font, FontRef, FontVec, PxScale, ScaleFont, Point, Glyph};

const TEXT: &str = "test_button";
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
use wgpu::{BlendFactor, BlendOperation, RenderPipeline, BindGroup, BindGroupLayout, ShaderModule};
use super::shape::*;
use super::mywgpu;

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
        // 渲染文字纹理配置
        let texture_state = TextureState::create_texture_group(&self);

        // let rect = Rectangle::new(20.0, 10.0, 400, 40);
        let font_shader = Shader::create_font_shader(&self);
        let shape_shader = Shader::create_shape_shader(&self);

        let render_pipeline =
            PipelineState::create_pipeline_state(&self, &font_shader, RenderType::Texture(&texture_state.texture_bind_group_layout));
        let vertex_buffer = VertexBuffer::create_tex_vertex_buf(&self, &rect);

        let shape_pipeline =
            PipelineState::create_pipeline_state(&self, &shape_shader, RenderType::Shape);
        let shape_vertex_buffer = VertexBuffer::create_shape_vertex_buf(&self, &rect);
        let border_pipeline =
            PipelineState::create_pipeline_state(&self, &shape_shader, RenderType::Border);
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
        render_pass.set_pipeline(&shape_pipeline);
        render_pass.set_vertex_buffer(0, shape_vertex_buffer.vertex_buffer.slice(..));
        render_pass.set_index_buffer(shape_vertex_buffer.index_buffer.slice(..));
        render_pass.draw_indexed(0..shape_vertex_buffer.num_indices, 0, 0..1);

        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, &texture_state.diffuse_bind_group, &[]); // NEW!
        render_pass.set_vertex_buffer(0, vertex_buffer.vertex_buffer.slice(..));
        render_pass.set_index_buffer(vertex_buffer.index_buffer.slice(..));
        render_pass.draw_indexed(0..vertex_buffer.num_indices, 0, 0..1);

        render_pass.set_pipeline(&border_pipeline);
        render_pass.set_vertex_buffer(0, boder_vertex_buffer.vertex_buffer.slice(..));
        render_pass.set_index_buffer(boder_vertex_buffer.index_buffer.slice(..));
        render_pass.draw_indexed(0..boder_vertex_buffer.num_indices, 0, 0..1);
        if self.use_complex {
            render_pass.set_pipeline(&shape_pipeline);
            render_pass.set_vertex_buffer(0, shape_vertex_buffer.vertex_buffer.slice(..));
            render_pass.set_index_buffer(shape_vertex_buffer.index_buffer.slice(..));
            render_pass.draw_indexed(0..shape_vertex_buffer.num_indices, 0, 0..1);
        }
        std::mem::drop(render_pass);
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

pub struct VertexBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl<'a> VertexBuffer {
    pub fn default(globe_state: &'a GlobeState, rect: &'a Rectangle, indices: &'a [u16], test_color: RGBA) -> Self {
        let vect = rect.to_buff(globe_state.sc_desc.width, globe_state.sc_desc.height, test_color);
        let vertex_buffer = globe_state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vect.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = globe_state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let num_indices = indices.len() as u32;
        Self {
            vertex_buffer,
            num_indices,//11
            index_buffer,
        }
    }
    pub fn create_shape_vertex_buf(globe_state: &'a GlobeState, rect: &'a Rectangle) -> Self {
        let test_color = RGBA([0.5, 0.0, 0.5, 0.5]);
        let indices: &[u16] = &[0, 2, 1, 3];
        Self::default(globe_state, rect, indices, test_color)
    }

    pub fn create_border_vertex_buf(globe_state: &'a GlobeState, rect: &'a Rectangle) -> Self {
        let test_color = RGBA([0.5, 0.0, 0.5, 1.0]);
        let indices: &[u16] = &[0, 1, 3, 2, 0];
        Self::default(globe_state, rect, indices, test_color)
    }

    pub fn create_tex_vertex_buf(globe_state: &'a GlobeState, rect: &'a Rectangle) -> Self {
        let vect = rect.to_tex(globe_state.sc_desc.width, globe_state.sc_desc.height);

        let indices: &[u16] = &[0, 2, 1, 3];
        let vertex_buffer = globe_state.device
            .create_buffer_init(&mywgpu::description::
            create_buffer_init_descriptor(
                bytemuck::cast_slice(vect.as_slice()), wgpu::BufferUsage::VERTEX)
            );
        let index_buffer = globe_state.device.create_buffer_init(
            &mywgpu::description::create_buffer_init_descriptor(
                bytemuck::cast_slice(indices), wgpu::BufferUsage::INDEX)
        );
        let num_indices = indices.len() as u32;
        Self {
            vertex_buffer,
            num_indices,//11
            index_buffer,
        }
    }
}

pub struct Shader {
    pub vs_module: ShaderModule,
    pub fs_module: ShaderModule,
}

impl<'a> Shader {
    pub fn create_font_shader(globe_state: &'a GlobeState) -> Self {
        let vs_module = globe_state.device
            .create_shader_module(wgpu::include_spirv!("../../shader_c/font.vert.spv"));
        let fs_module = globe_state.device
            .create_shader_module(wgpu::include_spirv!("../../shader_c/font.frag.spv"));

        Self {
            vs_module,
            fs_module,
        }
    }

    pub fn create_shape_shader(globe_state: &'a GlobeState) -> Self {
        let vs_module = globe_state.device
            .create_shader_module(wgpu::include_spirv!("../../shader_c/rect.vert.spv"));
        let fs_module = globe_state.device
            .create_shader_module(wgpu::include_spirv!("../../shader_c/rect.frag.spv"));

        Self {
            vs_module,
            fs_module,
        }
    }
}

pub struct TextureState {
    pub texture_bind_group_layout: BindGroupLayout,
    pub diffuse_bind_group: BindGroup,
}

pub struct TextureBuffer<'a> {
    pub x: u32,
    pub y: u32,
    pub buf: &'a [u8],
}

impl<'a> TextureState {
    pub fn default(globe_state: &'a GlobeState, texture_buf: &'a TextureBuffer) -> Self {
        let texture_size = mywgpu::texture::create_texture_size(texture_buf.x, texture_buf.y);
        let diffuse_texture = globe_state.device.create_texture(
            &mywgpu::texture::create_texture_descriptor(&texture_size)
        );
        globe_state.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            mywgpu::texture::create_texture_copy_view(&diffuse_texture),
            // The actual pixel data
            // diffuse_rgba,
            texture_buf.buf,
            // The layout of the texture
            mywgpu::texture::create_texture_data_layout(texture_buf.x, texture_buf.y),
            texture_size,
        );

        /// 默认纹理渲染配置
        let diffuse_texture_view = diffuse_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = globe_state.device.create_sampler(&mywgpu::description::create_sample_descriptor());
        let texture_bind_group_layout = globe_state.device.create_bind_group_layout(
            &mywgpu::description::create_bind_group_layout_descriptor()
        );
        /// 描述纹理顶点数据布局,用于着色器识别数据
        let diffuse_bind_group = globe_state.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        Self {
            texture_bind_group_layout,
            diffuse_bind_group,
        }
    }

    pub fn create_texture_group(globe_state: &'a GlobeState) -> Self {
        let (x, y, buf) = draw_image(65.0);
        let dimensions = (x, y);
        println!("dimensions:{:?}", dimensions);
        let texture_buf = TextureBuffer { x, y, buf: buf.as_slice() };
        Self::default(globe_state, &texture_buf)
    }
}

fn draw_image(f_scale: f32) -> (u32, u32, Vec<u8>) {
    let font = FontRef::try_from_slice(include_bytes!("../../shader_c/SourceHanSansCN-Regular.otf")).unwrap();

    // The font size to use
    let scale = PxScale::from(f_scale);

    let scaled_font = font.as_scaled(scale);

    let mut glyphs = Vec::new();
    layout_paragraph(scaled_font, point(20.0, 20.0), 9999.0, TEXT, &mut glyphs);

    // Use a dark red colour
    let colour = (150, 0, 0);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };
    println!("gl x: {} gly: {}", glyphs_width, glyphs_height);

    // Create a new rgba image with some padding
    // let mut image = DynamicImage::new_rgba8(glyphs_width + 20, glyphs_height-15).to_rgba8();
    let size = (glyphs_width + 20) * (glyphs_height);
    let mut bufs1 = vec![0; (size * 4) as usize];

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            println!("max y:{}", bounds.min.y);
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                // println!("x: {} y: {}", x, y);
                let index = x + bounds.min.x as u32 - 20 + (glyphs_width + 20) * (y + bounds.min.y as u32 - 29);
                bufs1[(index * 4) as usize] = colour.0;
                bufs1[(index * 4) as usize + 1] = colour.1;
                bufs1[(index * 4) as usize + 2] = colour.2;
                bufs1[(index * 4) as usize + 3] = (v * 255.0) as u8;
            });
        }
    }

    // println!("bufst{:?}",bufst.as_slice());
    (glyphs_width + 20, glyphs_height, bufs1)
}

pub fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}
