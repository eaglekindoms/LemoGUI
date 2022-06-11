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

unsafe fn create_vertex_buffer(gl: &glow::Context) -> (NativeBuffer, NativeVertexArray) {
    // This is a flat array of f32s that are to be interpreted as vec2s.
    let triangle_vertices = [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32];
    let triangle_vertices_u8: &[u8] = core::slice::from_raw_parts(
        triangle_vertices.as_ptr() as *const u8,
        triangle_vertices.len() * core::mem::size_of::<f32>(),
    );

    // We construct a buffer and upload the data
    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle_vertices_u8, glow::STATIC_DRAW);

    // We now construct a vertex array to describe the format of the input buffer
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

    (vbo, vao)
}

fn set_uniform(gl: &glow::Context, program: NativeProgram, name: &str, value: f32) {
    unsafe {
        let uniform_location = gl.get_uniform_location(program, name);
        // See also `uniform_n_i32`, `uniform_n_u32`, `uniform_matrix_4_f32_slice` etc.
        gl.uniform_1_f32(uniform_location.as_ref(), value)
    }
}
