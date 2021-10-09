
struct VertexInput {
	[[location(0)]] size: vec2<f32>;
    [[location(1)]] pos: vec2<f32>;
    [[location(2)]] borderColor: vec4<f32>;
    [[location(3)]] frameColor: vec4<f32>;
    [[location(4)]] is_round_or_border: vec2<u32>;
    [[builtin(vertex_index)]] gl_VertexIndex: u32;
};

struct VertexOutput {
	[[location(0)]] pos: vec2<f32>;
    [[location(1)]] size: vec2<f32>;
    [[location(2)]] radius: f32;
    [[location(3)]] borderWidth: f32;
    [[location(4)]] borderColor: vec4<f32>;
    [[location(5)]] frameColor: vec4<f32>;
    [[builtin(position)]] gl_Position: vec4<f32>;
};
// var gl_VertexIndex: i32;

[[stage(vertex)]]
fn vs_main(input: VertexInput) -> VertexOutput  {
	var out: VertexOutput;

    let gl_VertexIndex = i32(input.gl_VertexIndex);
    var positions: array<vec2<f32>,4u> =
                    array<vec2<f32>,4u>(vec2<f32>(-1.0, 1.0),
                                        vec2<f32>(1.0, 1.0),
                                        vec2<f32>(-1.0, -1.0),
                                        vec2<f32>(1.0, -1.0));
    let coord: vec2<f32> = positions[gl_VertexIndex];
    out.gl_Position = vec4<f32>(coord.x, coord.y, 0.0, 1.0);
	out.pos = coord-input.pos;
	out.size = input.size;
	out.borderColor = input.borderColor;
	out.frameColor = input.frameColor;

	if (input.is_round_or_border[0u] == 0u){
        out.radius = 0.0;
    } else {
        out.radius = 0.03;
    }
    if (input.is_round_or_border[1u] == 0u){
        out.borderWidth = 0.0;
    } else {
        out.borderWidth = 0.01;
    }

    return out;
}

// Rounded rect distance function
fn udRoundRect(pos: vec2<f32>, temp_size: vec2<f32>, radius: f32) -> f32
{
    return length(max(abs(pos) - temp_size, vec2<f32>(0.0))) - radius;
}

// smoothstep第一个参数表示边缘虚化范围，为0.0时无虚化
// smoothstep第二个参数表示保留范围，当其大于参数一时保留范围为边框
// 即第一个确定取值，第二个确定取值范围
fn renderRectFrame(pos2: vec2<f32>, temp_size2: vec2<f32>, radius2: f32) -> f32 {
    return 1.0 - smoothStep(0.0, 0.006, udRoundRect(pos2, temp_size2, radius2));
}

[[stage(fragment)]]
fn fs_main(input: VertexOutput) -> [[location(0)]] vec4<f32>  {
	var frameAlpha: f32;
	var size: vec2<f32>;
	size = input.size * 0.5  - input.radius;
	frameAlpha = renderRectFrame(input.pos, size, input.radius);
	var fragColor: vec4<f32> = vec4<f32>(0.0);
	fragColor = mix(fragColor, input.frameColor, vec4<f32>(frameAlpha));
    return fragColor;
}