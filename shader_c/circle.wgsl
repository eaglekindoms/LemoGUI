
struct VertexInput {
	[[location(0)]] pos: vec2<f32>;
	[[location(1)]] color: vec4<f32>;
	[[location(2)]] radius: f32;
	[[location(3)]] edge: u32;
	[[builtin(vertex_index)]] gl_VertexIndex: u32;
};

struct VertexOutput {
    [[builtin(position)]] gl_Position: vec4<f32>;
	[[location(0)]] pos: vec2<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] radius: f32;
    [[location(3)]] edge: f32;
};
var gl_VertexIndex: i32;

[[stage(vertex)]]
fn vs_main(input: VertexInput) -> VertexOutput  {
	var out: VertexOutput;

    out.pos = input.pos;
    out.color = input.color;
    out.radius = input.radius;
    out.edge = f32(input.edge);

    gl_VertexIndex = i32(input.gl_VertexIndex);
    var positions: array<vec2<f32>,4u> =
                    array<vec2<f32>,4u>(vec2<f32>(-1.0, 1.0),
                                        vec2<f32>(1.0, 1.0),
                                        vec2<f32>(-1.0, -1.0),
                                        vec2<f32>(1.0, -1.0));
    let coord: vec2<f32> = positions[gl_VertexIndex];
    out.gl_Position= vec4<f32>(coord, 0.0, 1.0);

    return out;
}

fn circle(uvs: vec2<f32>, pos: vec2<f32>, rad: f32) -> f32 {
    var d: f32;
    var t: f32;

    d = (length(pos - uvs) - rad);
    t = clamp(d, 0.0, 1.0);
    return (1.0 - t);
}

fn polygon(uvs1:vec2<f32>, pos1:vec2<f32>, radius: f32, n: f32) -> f32 {
    var d1: f32;
    var angle: f32;
    var r: f32;
    var b: f32;

    d1 = (length(pos1 - uvs1) / radius);
    angle = atan2((uvs1[0u] - pos1[0u]), ( uvs1[1u] - pos1[1u]));
    r = (6.28318530718 / n);
    b = (cos(((floor((0.5 + (angle / r))) * r) - angle)) * d1);
    return (1.0 - step(0.8 , b));
}


[[stage(fragment)]]
fn fs_main(input: VertexOutput) -> [[location(0)]] vec4<f32>  {

    var intensity: f32;

    if ((input.edge > 2.0)) {
        intensity = polygon(input.gl_Position.xy, input.pos, input.radius, input.edge);
    } else {
        intensity = circle(input.gl_Position.xy, input.pos, input.radius);
    }
    if ((intensity > 0.0)) {
         return input.color;
    }
   return vec4<f32>(0.0,0.0,0.0,0.0);
}