#version 450

layout(location=0) in vec2 fragCoord;// 固定值
layout(location=1) in vec2 v_size;//
layout(location=2) in float v_radius;// 矩形圆角大小
layout(location=3) in float v_borderWidth;// 矩形边框宽度
layout(location=4) in vec4 v_borderColor;// 矩形边框颜色
layout(location=6) in vec4 v_frameColor;// 矩形填充颜色
layout(location=7) in vec2 v_pos;

layout(location=0) out vec4 fragColor;

// Rounded rect distance function
float udRoundRect(vec2 pos, vec2 temp_size, float radius)
{
    return length(max(abs(pos) - temp_size, 0.0)) - radius;
}
// smoothstep第一个参数表示边缘虚化范围，为0.0时无虚化
// smoothstep第二个参数表示保留范围，当其大于参数一时保留范围为边框
// 即第一个确定取值，第二个确定取值范围
float renderRectFrame(vec2 pos, vec2 temp_size, float radius){
    return 1- smoothstep(0.0, 0.006, udRoundRect(pos, temp_size, radius));
}
float renderRectBorder(vec2 pos, vec2 temp_size, float radius, float border_width){
    if (border_width<0.02){
        if (border_width>0.002){
            return 1- smoothstep(border_width/2, border_width, abs(udRoundRect(pos, temp_size, radius)));
        } else {
            return 1-smoothstep(0.001, 0.002, abs(udRoundRect(pos, temp_size, radius)));
        }
    } else {
        return 1- smoothstep(0.01, 0.02, abs(udRoundRect(pos, temp_size, radius)));
    }
}
void main(){
    // 填充
    float frameAlpha;
    float borderAlpha;
    vec2 pos= v_pos;
    vec2 size= v_size*0.5  - v_radius;
    frameAlpha = renderRectFrame(pos, size, v_radius);
    fragColor = mix(fragColor, v_frameColor, frameAlpha);

    //    // 边框
    //    float border_width = v_borderWidth;
    //    if (border_width!=0){
    //        borderAlpha = renderRectBorder(pos, size, v_radius, border_width);
    //        fragColor = mix(fragColor, v_borderColor, borderAlpha);
    //    }
    //        fragColor=vec4(1);
}

