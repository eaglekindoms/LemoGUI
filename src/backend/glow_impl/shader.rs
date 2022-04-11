use glow::*;

#[derive(Debug)]
pub struct Pipeline {
    shader: glow::NativeProgram,
}

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub fn create_program(gl: &glow::Context) -> glow::NativeProgram {
    let program;
    unsafe {
        program = gl.create_program().expect("Cannot create program");
    }
    return program;
}

pub fn create_shader(
    gl: &glow::Context,
    program: &glow::NativeProgram,
    shader_type: ShaderType,
    shader_src: &str,
) {
    let s_type = match shader_type {
        ShaderType::Vertex => glow::VERTEX_SHADER,
        ShaderType::Fragment => glow::FRAGMENT_SHADER,
    };
    unsafe {
        let shader = gl.create_shader(s_type).expect("Cannot create shader");
        gl.shader_source(shader, shader_src);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }
        gl.attach_shader(*program, shader);
        gl.link_program(*program);
        if !gl.get_program_link_status(*program) {
            panic!("{}", gl.get_program_info_log(*program));
        }
        gl.detach_shader(*program, shader);
        gl.delete_shader(shader);
    }
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
