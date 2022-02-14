use glow::HasContext;

use crate::graphic::base::RGBA;

/// openGL context
#[derive(Debug)]
pub struct GLGPUContext {
    pub gl_context: glow::Context,
    pub gl_window: glutin::RawContext<glutin::PossiblyCurrent>,
}

impl GLGPUContext {
    pub fn new<T>(
        window_builder: winit::window::WindowBuilder,
        event_loop: &winit::event_loop::EventLoop<T>,
    ) -> (Self, winit::window::Window) {
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

        let (gl_window, window) = unsafe {
            use glow::HasContext as _;
            gl.enable(glow::FRAMEBUFFER_SRGB);
            glutin_window.split()
        };
        return (
            GLGPUContext {
                gl_context: gl,
                gl_window,
            },
            window,
        );
    }
    pub fn clear_frame(&self, color: RGBA) {
        unsafe {
            self.gl_context
                .clear_color(color.0, color.1, color.2, color.3);
            self.gl_context.clear(glow::COLOR_BUFFER_BIT);
            self.gl_window.swap_buffers().unwrap();
        }
    }
}
