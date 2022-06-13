use crate::backend::glow_impl::GLPipeline;
use glow::HasContext;
use std::collections::HashMap;
use std::sync::Arc;

use crate::graphic::base::*;
use crate::widget::ComponentModel;

/// openGL context
#[derive(Debug)]
pub struct GLGPUContext {
    pub gl_context: Arc<glow::Context>,
    gl_window: glutin::RawContext<glutin::PossiblyCurrent>,
    pub window: Arc<winit::window::Window>,
    pub pipelines: HashMap<ShapeType, GLPipeline>,
}

impl GLGPUContext {
    pub fn new_with_builder<T>(
        window_builder: winit::window::WindowBuilder,
        event_loop: &winit::event_loop::EventLoop<T>,
    ) -> (Self, Arc<winit::window::Window>) {
        let glutin_window = unsafe {
            glutin::ContextBuilder::new()
                .with_depth_buffer(0)
                .with_srgb(true)
                .with_stencil_buffer(0)
                .with_vsync(true)
                .build_windowed(window_builder, event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };

        let gl =
            unsafe { glow::Context::from_loader_function(|s| glutin_window.get_proc_address(s)) };
        let (gl_window, window) = unsafe { glutin_window.split() };
        let gl_context = Arc::new(gl);
        config_gl(&gl_context);
        let pipelines = create_pipelines(&gl_context);
        let window = Arc::new(window);
        return (
            GLGPUContext {
                gl_context,
                gl_window,
                window: Arc::clone(&window),
                pipelines,
            },
            window,
        );
    }

    pub fn clear_frame(&self, color: RGBA) {
        unsafe {
            // self.gl_context.disable(glow::SCISSOR_TEST);
            self.gl_context
                .clear_color(color.0, color.1, color.2, color.3);
            self.gl_context.clear(glow::COLOR_BUFFER_BIT);
            self.gl_window.swap_buffers().unwrap();
        }
    }
    // 更新交换缓冲区
    pub fn update_surface_configure<P: Into<Point<u32>>>(&mut self, size: P) {
        let size = size.into();
        unsafe {
            self.gl_context.viewport(0, 0, size.x as i32, size.y as i32);
        }
        self.gl_window.swap_buffers().unwrap();
    }
    /// 显示图形内容
    pub fn present<C, M>(&mut self, container: &mut C, font_map: &mut GCharMap)
    where
        C: ComponentModel<M> + 'static,
        M: 'static + std::fmt::Debug,
    {
    }
}

fn create_pipelines(context: &Arc<glow::Context>) -> HashMap<ShapeType, GLPipeline> {
    let mut pipelines = HashMap::with_capacity(4);
    pipelines.insert(ShapeType::POINT, GLPipeline::new::<PointVertex>(context));
    pipelines.insert(ShapeType::ROUND, GLPipeline::new::<RectVertex>(context));
    pipelines.insert(ShapeType::Circle, GLPipeline::new::<CircleVertex>(context));
    return pipelines;
}

fn config_gl(context: &Arc<glow::Context>) {
    unsafe {
        // Enable auto-conversion from/to sRGB
        log::info!("Enabling GLES debug output");
        context.enable(glow::DEBUG_OUTPUT);
        context.debug_message_callback(gl_debug_message_callback);
        context.enable(glow::FRAMEBUFFER_SRGB);

        // Enable alpha blending
        context.enable(glow::BLEND);
        // gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        context.blend_equation_separate(glow::FUNC_ADD, glow::FUNC_ADD);
        context.blend_func_separate(
            glow::SRC_ALPHA,
            glow::ONE_MINUS_SRC_ALPHA,
            glow::ONE,
            glow::ONE_MINUS_SRC_ALPHA,
        );
        // gl.blend_equation(glow::FUNC_ADD); // default
        // Disable multisampling by default
        context.disable(glow::MULTISAMPLE);
    }
}

fn gl_debug_message_callback(source: u32, gltype: u32, id: u32, severity: u32, message: &str) {
    let source_str = match source {
        glow::DEBUG_SOURCE_API => "API",
        glow::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
        glow::DEBUG_SOURCE_SHADER_COMPILER => "ShaderCompiler",
        glow::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
        glow::DEBUG_SOURCE_APPLICATION => "Application",
        glow::DEBUG_SOURCE_OTHER => "Other",
        _ => unreachable!(),
    };

    let log_severity = match severity {
        glow::DEBUG_SEVERITY_HIGH => log::Level::Error,
        glow::DEBUG_SEVERITY_MEDIUM => log::Level::Warn,
        glow::DEBUG_SEVERITY_LOW => log::Level::Info,
        glow::DEBUG_SEVERITY_NOTIFICATION => log::Level::Trace,
        _ => unreachable!(),
    };

    let type_str = match gltype {
        glow::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behavior",
        glow::DEBUG_TYPE_ERROR => "Error",
        glow::DEBUG_TYPE_MARKER => "Marker",
        glow::DEBUG_TYPE_OTHER => "Other",
        glow::DEBUG_TYPE_PERFORMANCE => "Performance",
        glow::DEBUG_TYPE_POP_GROUP => "Pop Group",
        glow::DEBUG_TYPE_PORTABILITY => "Portability",
        glow::DEBUG_TYPE_PUSH_GROUP => "Push Group",
        glow::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
        _ => unreachable!(),
    };

    log::log!(
        log_severity,
        "GLES: [{}/{}] ID {} : {}",
        source_str,
        type_str,
        id,
        message
    );

    if cfg!(debug_assertions) && log_severity == log::Level::Error {
        std::process::exit(1);
    }
}
