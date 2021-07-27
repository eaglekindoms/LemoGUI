#version 450

layout(location=0) in vec2 a_pos;
layout(location=1) in vec4 a_color;
layout(location=2) in float a_radius;

layout(location=0) out vec4 v_color;
layout(location=1) out float v_radius;
layout(location=2) out vec2 v_pos;

const vec2 positions[4] = vec2[4](
vec2(-1.0, 1.0),
vec2(1.0, 1.0),
vec2(-1.0, -1.0),
vec2(1.0, -1.0));
void main() {
//    v_position =  positions[gl_VertexIndex];
    v_pos=positions[gl_VertexIndex] - a_pos;
    v_color = a_color;
    v_radius=a_radius;
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
}
