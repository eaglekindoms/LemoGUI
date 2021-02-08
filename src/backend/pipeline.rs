use std::iter;
use ab_glyph::{point, Font, FontRef, FontVec, PxScale, ScaleFont, Point, Glyph};

const TEXT: &str = "测s会更加看看灵颗粒,,,,";

use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use wgpu::{BlendFactor, BlendOperation};

#[path = "../wgpu/libs.rs"]
mod mywgpu;

use mywgpu::Vertex;

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0] }, // B  1
    Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0] }, // A   2
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] }, // C 3
    Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] }, // D  4
    // Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641] }, // E
];

const INDICES: &[u16] = &[0, 2, 1, 3];

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    render_pipeline: wgpu::RenderPipeline,
    size: winit::dpi::PhysicalSize<u32>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
}

impl State {
    async fn new(window: &Window) -> Self {
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
        // ---
        let (x, y, buf) = draw_image(265.0);
        // use image::GenericImageView;
        let dimensions = (x, y);
        println!("dimensions:{:?}", dimensions);
        //\\\
        let texture_size = mywgpu::texture::create_texture_size(x, y);
        let diffuse_texture = device.create_texture(
            &mywgpu::texture::create_texture_descriptor(&texture_size)
        );
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            mywgpu::texture::create_texture_copy_view(&diffuse_texture),
            // The actual pixel data
            // diffuse_rgba,
            buf.as_slice(),
            // The layout of the texture
            mywgpu::texture::create_texture_data_layout(x, y),
            texture_size,
        );

        // We don't need to configure the texture view much, so let's
        // let wgpu define it.
        let diffuse_texture_view = diffuse_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&mywgpu::description::create_sample_descriptor());

        let texture_bind_group_layout = device.create_bind_group_layout(
            &mywgpu::description::create_bind_group_layout_descriptor()
        );
        let diffuse_bind_group = device.create_bind_group(
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
        let vs_module = device
            .create_shader_module(wgpu::include_spirv!("../resources/shader_c/font.vert.spv"));
        let fs_module = device
            .create_shader_module(wgpu::include_spirv!("../resources/shader_c/font.frag.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex_stage: mywgpu::description::create_shader_descriptor(&vs_module),
                fragment_stage: Some(mywgpu::description::create_shader_descriptor(&fs_module)),
                rasterization_state: Some(mywgpu::description::create_rasterization_state_descriptor()),
                primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
                color_states: &[mywgpu::description::be_blend_color_state_descriptor(sc_desc.format)],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[Vertex::desc()],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        let (t_w, t_h) = (200.0, 20.0);
        let (x, y) = (12, 12);
        // gl
        let vect: &[Vertex] = &[
            Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0] }, // B  1
            Vertex { position: [-0.5 + 2.0 * t_w / sc_desc.width as f32, 0.5, 0.0], tex_coords: [1.0, 0.0] }, // A   2
            Vertex { position: [-0.5, 0.5 - 2.0 * t_h / sc_desc.height as f32, 0.0], tex_coords: [0.0, 1.0] }, // C 3
            Vertex { position: [-0.5 + 2.0 * t_w / sc_desc.width as f32, 0.5 - 2.0 * t_h / sc_desc.height as f32, 0.0], tex_coords: [1.0, 1.0] }, // D  4
            // Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641] }, // E
        ];
        let vertex_buffer = device
            .create_buffer_init(&mywgpu::description::create_buffer_init_descriptor(bytemuck::cast_slice(vect), wgpu::BufferUsage::VERTEX)
            );
        let index_buffer = device.create_buffer_init(&mywgpu::description::create_buffer_init_descriptor(bytemuck::cast_slice(INDICES), wgpu::BufferUsage::INDEX)
        );
        let num_indices = INDICES.len() as u32;
        //\\\

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipeline, //11
            vertex_buffer,//11
            index_buffer,//11
            num_indices,//11
            diffuse_bind_group,//11
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let (x, y, w, h) = (12, 12, 600, 200);
            let (t_x, t_y, t_w, t_h) =
                (2.0 * x as f32 / self.sc_desc.width as f32 - 1.0,
                 1.0 - 2.0 * y as f32 / self.sc_desc.height as f32,
                 2.0 * w as f32 / self.sc_desc.width as f32,
                 2.0 * h as f32 / self.sc_desc.height as f32);

            let vect: &[Vertex] = &[
                Vertex { position: [t_x, t_y, 0.0], tex_coords: [0.0, 0.0] }, // B  1
                Vertex { position: [t_x + t_w, t_y, 0.0], tex_coords: [1.0, 0.0] }, // A   2
                Vertex { position: [t_x, t_y - t_h, 0.0], tex_coords: [0.0, 1.0] }, // C 3
                Vertex { position: [t_x + t_w, t_y - t_h, 0.0], tex_coords: [1.0, 1.0] }, // D  4
                // Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641] }, // E
            ];
            let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vect),
                usage: wgpu::BufferUsage::VERTEX,
            });
            let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsage::INDEX,
            });
            // 一帧
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // NEW!
            // one
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..2);
            // two
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..2);
        }

        self.queue.submit(iter::once(encoder.finish()));


        Ok(())
    }

    pub fn render1(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    use futures::executor::block_on;

    // Since main can't be async, we're going to need to block
    let mut state = block_on(State::new(&window));
    #[path="../wgpu/globeSetting.rs"]
    mod setting;
    let pip = setting::PipelineState
    ::create_render_pipeline(&block_on(setting::GlobeState::new(&window)));


    // let mut state1 = block_on(State1::new(&window));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        },
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged
                        {
                            new_inner_size, ..
                        } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                };
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

fn draw_image(f_scale: f32) -> (u32, u32, Vec<u8>) {
    let font = FontRef::try_from_slice(include_bytes!("../../fonts/SourceHanSansCN-Regular.otf")).unwrap();

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
    let mut bufs1: Vec<u8> = vec![0; (size * 4) as usize];

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
