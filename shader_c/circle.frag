#version 450

layout(location=0) in vec4 v_color;
layout(location=1) in float v_radius;
layout(location=2) in vec2 v_pos;

layout(location=0) out vec4 f_color;

vec4 circle(vec2 pos, float rad, vec4 color) {
    float d = length(pos) - rad;
    float t = clamp(d, 0.0, 1.0);
    return vec4(color.xyz, 1.0 - t);
}
float sdCircle(vec2 p, float r)
{
    float d =  length(p)-r;
    return 1-sign(d);
}

void main() {
    float frameAlpha=sdCircle(v_pos, v_radius);
    //        vec4 frameColor=circle(v_pos,  v_radius, v_color);
    //    f_color=mix(f_color,frameColor,frameColor.a);
    if (frameAlpha>0){
        f_color =  v_color;
    }
    //    f_color = mix(f_color, v_color, frameAlpha);
}
