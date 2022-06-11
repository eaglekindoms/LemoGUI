#version 450
#extension GL_KHR_vulkan_glsl : enable

layout(location=0) in vec2 a_pos;
layout(location=1) in vec4 a_color;
layout(location=2) in float a_radius;
layout(location=3) in uint a_edge;

layout(location=0) out vec4 v_color;
layout(location=1) out vec2 v_pos;
layout(location=2) out float v_edge;
layout(location=3) out float v_radius;

uniform uvec2 u_screen_size;

const vec2 positions[4] = vec2[4](
vec2(-1.0, 1.0),
vec2(1.0, 1.0),
vec2(-1.0, -1.0),
vec2(1.0, -1.0));
void main() {
    v_pos =  vec2(a_pos.x / u_screen_size.x, a_pos.y / u_screen_size.y);
    v_color = a_color;
    v_radius = a_radius/ u_screen_size.x;
    v_edge = float(a_edge);
    gl_Position = vec4(positions[gl_VertexID], 0.0, 1.0);
}
