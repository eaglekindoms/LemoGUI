use wgpu::{Device, ShaderModule};

pub struct Shader {
    pub vs_module: ShaderModule,
    pub fs_module: ShaderModule,
}
