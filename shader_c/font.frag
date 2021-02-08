// shader.frag
#version 450

// Changed
layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

// NEW!
layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    // Changed
    vec4 texColor = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords)* vec4(1.0,1.0,1.0, 1.0);
    //if(texColor.a < 0.1)
    //    discard;
    f_color = texColor;
}
