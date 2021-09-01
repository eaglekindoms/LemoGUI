
struct VertexInput {
	[[location(0)]] pos: vec2<f32>;
	[[location(1)]] size: vec2<f32>;
	[[builtin(vertex_index)]] gl_VertexIndex: u32;
};

struct VertexOutput {
    [[builtin(position)]] gl_Position: vec4<f32>;
	[[location(0)]] v_tex_coords: vec2<f32>;
};
var gl_VertexIndex: i32;

[[stage(vertex)]]
fn vs_main(input: VertexInput) -> VertexOutput  {
	var out: VertexOutput;

    gl_VertexIndex = i32(input.gl_VertexIndex);
    var positions: array<vec2<f32>,4u> =
                    array<vec2<f32>,4u>(vec2<f32>(-1.0, 1.0),
                                        vec2<f32>(1.0, 1.0),
                                        vec2<f32>(-1.0, -1.0),
                                        vec2<f32>(1.0, -1.0));
	var tex_coords: array<vec2<f32>,4u> =
					array<vec2<f32>,4u>(vec2<f32>(0.0, 0.0),
										vec2<f32>(1.0, 0.0),
										vec2<f32>(0.0, 1.0),
										vec2<f32>(1.0, 1.0));
    var coord: vec2<f32>;
    var pos: vec2<f32> = input.pos;
    var size: vec2<f32> = input.size;
	if (gl_VertexIndex == 0) {
	    coord = pos;
	}
    if (gl_VertexIndex == 1) {
        coord = vec2<f32>((pos[0u]+ size[0u]), pos[1u]);
    }
    if (gl_VertexIndex == 2) {
        coord = vec2<f32>(pos[0u], (pos[1u] - size[1u]));
    }
    if (gl_VertexIndex == 3) {
        coord = vec2<f32>((pos[0u] + size[0u]), (pos[1u] - size[1u]));
    }

    out.gl_Position = vec4<f32>(coord.x, coord.y, 0.0, 1.0);
	out.v_tex_coords = tex_coords[gl_VertexIndex];

    return out;
}

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(input: VertexOutput) -> [[location(0)]] vec4<f32> {

    var texColor: vec4<f32>;
    texColor = textureSample(t_diffuse, s_diffuse, input.v_tex_coords) * vec4<f32>(1.0, 1.0, 1.0, 1.0);
    return texColor;
}
