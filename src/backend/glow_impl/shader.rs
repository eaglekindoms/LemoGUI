use glow::*;

#[derive(Debug)]
pub struct GLShader {
    pub vert: NativeShader,
    pub frag: NativeShader,
}

impl GLShader {
    pub fn new(gl: &glow::Context, vert_src: &str, frag_src: &str) -> Self {
        GLShader {
            vert: compile_shader(gl, VERTEX_SHADER, vert_src),
            frag: compile_shader(gl, FRAGMENT_SHADER, frag_src),
        }
    }
    pub fn link_program(&self, gl: &glow::Context) -> glow::NativeProgram {
        let program;
        unsafe {
            program = gl.create_program().expect("Cannot create program");
            gl.attach_shader(program, self.vert);
            gl.attach_shader(program, self.frag);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }
            gl.detach_shader(program, self.vert);
            gl.delete_shader(self.vert);
            gl.detach_shader(program, self.frag);
            gl.delete_shader(self.frag);
        }
        return program;
    }
}

fn compile_shader(gl: &glow::Context, shader_type: u32, shader_src: &str) -> NativeShader {
    unsafe {
        let shader = gl.create_shader(shader_type).expect("Cannot create shader");
        gl.shader_source(shader, shader_src);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }
        return shader;
    }
}

#[deprecated]
pub fn create_shader(gl: &glow::Context, vert_src: &str, frag_src: &str) -> glow::NativeProgram {
    let shader = GLShader::new(gl, vert_src, frag_src);
    let program = shader.link_program(gl);

    return program;
}
