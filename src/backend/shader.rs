use wgpu::ShaderModule;
use crate::backend::global_setting::GlobalState;

pub struct Shader {
    pub vs_module: ShaderModule,
    pub fs_module: ShaderModule,
}

impl<'a> Shader {
    pub fn create_font_shader(globe_state: &'a GlobalState) -> Self {
        let vs_module = globe_state.device
            .create_shader_module(&wgpu::include_spirv!("../../shader_c/font.vert.spv"));
        let fs_module = globe_state.device
            .create_shader_module(&wgpu::include_spirv!("../../shader_c/font.frag.spv"));

        Self {
            vs_module,
            fs_module,
        }
    }

    pub fn create_shape_shader(globe_state: &'a GlobalState) -> Self {
        let vs_module = globe_state.device
            .create_shader_module(&wgpu::include_spirv!("../../shader_c/rect.vert.spv"));
        let fs_module = globe_state.device
            .create_shader_module(&wgpu::include_spirv!("../../shader_c/rect.frag.spv"));

        Self {
            vs_module,
            fs_module,
        }
    }
}
