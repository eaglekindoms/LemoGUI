
struct VertexInput {
    @location(0) a_position: vec2<f32>,
    @location(1) a_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) v_color: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.v_color = input.a_color;
    out.position = vec4<f32>(input.a_position, 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.v_color;
}