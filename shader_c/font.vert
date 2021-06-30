// shader.vert
#version 450

layout(location=0) in vec2 a_pos;
layout(location=1) in vec2 a_size;
// Changed
layout(location=0) out vec2 v_tex_coords;

const vec2 tex_coords[4] = vec2[4](
vec2(0.0, 0.0),
vec2(1.0, 0.0),
vec2(0.0, 1.0),
vec2(1.0, 1.0));

const vec2 positions[4] = vec2[4](
vec2(-1.0, 1.0),
vec2(1.0, 1.0),
vec2(-1.0, -1.0),
vec2(1.0, -1.0));
void main() {
    // Changed
    v_tex_coords = tex_coords[gl_VertexIndex];
    vec2 pos= positions[gl_VertexIndex];
    if (pos==vec2(-1.0, 1.0)) pos = a_pos;
    if (pos==vec2(1.0, 1.0)) pos = vec2(a_pos.x+a_size.x, a_pos.y);
    if (pos==vec2(-1.0, -1.0)) pos = vec2(a_pos.x, a_pos.y-a_size.y);
    if (pos==vec2(1.0, -1.0)) pos = vec2(a_pos.x+a_size.x, a_pos.y-a_size.y);

    gl_Position =  vec4(pos, 0.0, 1.0);
}
