#version 450

layout(location=0) in vec4 v_color;
layout(location=1) in vec2 v_pos;
layout(location=2) in float v_edge;
layout(location=3) in float v_radius;

layout(location=0) out vec4 f_color;

// https://www.shadertoy.com/view/MssyRN
const float      PI = 3.14159265359;
const float  TWO_PI = 6.28318530718;

vec2 uv = vec2(0);

float polygon(vec2 uvs, vec2 pos, float radius, float n)
{
    float d = length(pos - uvs)/radius;
    float angle=atan(uvs.x-pos.x, uvs.y-pos.y);
    float r = TWO_PI / n;
    float b=  cos(floor(0.5 + angle / r) * r - angle)*d;
    return 1.- step(0.8, b);
}

float circle(vec2 uvs, vec2 pos, float rad) {
    float d = length(pos - uvs) - rad;
    float t = clamp(d, 0.0, 1.0);
    return 1.0 - t;
}
void main() {
    float intensity;
    if (v_edge>2){
        intensity = polygon(gl_FragCoord.xy, v_pos, v_radius, v_edge);
    } else {
        // Circle
        intensity = circle(gl_FragCoord.xy, v_pos, v_radius);
    }
    if (intensity>0.){
        f_color =v_color;
    }
}
