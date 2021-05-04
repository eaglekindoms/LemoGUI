use wgpu::{Device, ShaderModule};

pub struct Shader {
    pub vs_module: ShaderModule,
    pub fs_module: ShaderModule,
}

impl<'a> Shader {
    pub fn create_font_shader(device: &Device) -> Self {
        let vs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../shader_c/font.vert.spv"));
        let fs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../shader_c/font.frag.spv"));

        Self {
            vs_module,
            fs_module,
        }
    }

    pub fn create_shape_shader(device: &Device) -> Self {
        let vs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../shader_c/rect.vert.spv"));
        let fs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../shader_c/rect.frag.spv"));

        Self {
            vs_module,
            fs_module,
        }
    }

    pub fn create_round_shape_shader(device: &Device) -> Self {
        let vs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../shader_c/round_rect.vert.spv"));
        let fs_module = device
            .create_shader_module(&wgpu::include_spirv!("../../shader_c/round_rect.frag.spv"));

        Self {
            vs_module,
            fs_module,
        }
    }
}
