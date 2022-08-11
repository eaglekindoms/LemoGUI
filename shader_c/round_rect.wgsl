
struct VertexInput {
	@location(0) size: vec2<f32>,
    @location(1) pos: vec2<f32>,
    @location(2) borderColor: vec4<f32>,
    @location(3) rectColor: vec4<f32>,
    @location(4) is_round_or_border: vec2<u32>,
    @builtin(vertex_index) gl_VertexIndex: u32,
};

struct VertexOutput {
	@location(0) pos: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) is_round_or_border: vec2<u32>,
    @location(3) borderColor: vec4<f32>,
    @location(4) rectColor: vec4<f32>,
    @builtin(position) gl_Position: vec4<f32>,
};
// var gl_VertexIndex: i32;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput  {
	var out: VertexOutput;

    let gl_VertexIndex = i32(input.gl_VertexIndex);
    var positions: array<vec2<f32>,4u> =
                        array<vec2<f32>,4u>(vec2<f32>(-1.0, 1.0),
                                            vec2<f32>(1.0, 1.0),
                                            vec2<f32>(-1.0, -1.0),
                                            vec2<f32>(1.0, -1.0));
    let coord: vec2<f32> = positions[gl_VertexIndex];
    out.gl_Position= vec4<f32>(coord, 0.0, 1.0);
    let start_x: f32 =  input.pos.x;
    let start_y: f32 =  input.pos.y;
    let width: f32 =   input.size.x;
    let height: f32 =  input.size.y;
	out.pos = vec2<f32>(start_x + width / 2.0, start_y + height / 2.0);
	out.size = vec2<f32>(width, height);
	out.borderColor = input.borderColor;
	out.rectColor = input.rectColor;
	out.is_round_or_border = input.is_round_or_border;

    return out;
}

fn rect(uvs: vec2<f32>, pos: vec2<f32>, size: vec2<f32>, rad: f32) -> f32{
    var alpha: f32 = 0.0;
    let p_a: vec2<f32> = pos - size/2.0;
    let p_b: vec2<f32> = pos + size/2.0;
    let p_ar: vec2<f32> = p_a + vec2<f32>(rad);
    let p_br: vec2<f32> = p_b - vec2<f32>(rad);
    let p_a1: vec2<f32> = vec2<f32>(p_a.x + rad, p_b.y - rad);
    let p_b1: vec2<f32> = vec2<f32>(p_b.x - rad, p_a.y + rad);
    if (all(p_a < uvs)&&all(uvs < p_b)){
       alpha = 1.0;
       if((all(uvs < p_ar)&&distance(uvs,p_ar)>rad)
           ||(all(uvs > p_br)&&distance(uvs,p_br)>rad)
           ||((uvs.x<p_a1.x&&uvs.y>p_a1.y)&&(distance(uvs,p_a1)>rad))
           ||((uvs.x>p_b1.x&&uvs.y<p_b1.y)&&(distance(uvs,p_b1)>rad))){
           alpha=0.0;
       }
    }
    return alpha;
}
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32>  {
	var frameAlpha: f32;
	var size: vec2<f32>;
	var radius: f32;
	var border: f32 = 0.0;
	var fragColor: vec4<f32> = vec4<f32>(0.0);
	var borderColor: vec4<f32> = input.rectColor;
    size = input.size;

	if (input.is_round_or_border[0u]==0u){
	    radius = 0.0;
	}else{
	    radius = 8.0;
	}
	if (input.is_round_or_border[1u]==1u){
		border = rect(input.gl_Position.xy,input.pos ,size - vec2<f32>(1.0), radius);
		borderColor = input.borderColor;
	}

	frameAlpha = rect(input.gl_Position.xy,input.pos ,size ,radius);
	if (frameAlpha > 0.0&&border==0.0){
	    fragColor = borderColor;
    } else if(frameAlpha > 0.0&&border>0.0){
   	    fragColor = input.rectColor;
    }
    return fragColor;
}