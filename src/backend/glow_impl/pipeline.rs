use glow::*;

#[derive(Debug)]
pub struct Pipeline {
    shader: glow::NativeProgram,
}

pub enum ShaderType {
    Vertex,
    Fragment,
}


fn create_shader(
    gl: &glow::Context,
    shader_type: ShaderType,
    shader_src: &str,
) -> glow::NativeProgram {
    let program;
    let s_type = match shader_type {
        ShaderType::Vertex => glow::VERTEX_SHADER,
        ShaderType::Fragment => glow::FRAGMENT_SHADER,
    };
    unsafe {
        program = gl.create_program().expect("Cannot create program");
        let shader = gl.create_shader(s_type).expect("Cannot create shader");
        gl.shader_source(shader, shader_src);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }
        gl.attach_shader(program, shader);
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }
        gl.detach_shader(program, shader);
        gl.delete_shader(shader);
    }
    return program;
}