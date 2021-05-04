#version 450

//layout(location=0) in vec2 a_position;// 固定值

layout(location=1) in vec2 a_size;// 矩形大小
layout(location=2) in vec2 a_pos;// 矩形位置

layout(location=3) in vec4 a_borderColor;// 矩形边框颜色
layout(location=4) in vec4 a_frameColor;// 矩形填充颜色

layout(location=5) in float a_radius;// 矩形圆角大小
layout(location=6) in float a_borderWidth;// 矩形边框宽度

layout(location=0) out vec2 v_position;// 固定值
layout(location=1) out vec2 v_size;//
layout(location=2) out float v_radius;// 矩形圆角大小
layout(location=3) out float v_borderWidth;// 矩形边框宽度
layout(location=4) out vec4 v_borderColor;// 矩形边框颜色
layout(location=6) out vec4 v_frameColor;// 矩形填充颜色
layout(location=7) out vec2 v_pos;

const vec2 positions[4] = vec2[4](
vec2(1.0, 1.0),
vec2(-1.0, 1.0),
vec2(-1.0, -1.0),
vec2(1.0, -1.0));
//gl_VertexIndex
void main() {
    v_position =  positions[gl_VertexIndex];
    v_radius = a_radius;

    v_pos=v_position - a_pos;
    v_size = a_size - v_radius;
    v_borderWidth = a_borderWidth;
    v_borderColor = a_borderColor;
    v_frameColor = a_frameColor;
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
}