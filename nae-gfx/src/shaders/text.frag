#version 450
precision mediump float;

layout(location = 0) in vec4 v_color;
layout(location = 1) in vec2 v_texcoord;

layout(location = 0) out vec4 outColor;

layout(location = 0) uniform sampler2D u_texture;

void main() {
    float alpha = texture(u_texture, v_texcoord).r;
    if(alpha <= 0.0) {
        discard;
    }

    outColor = v_color * vec4(1.0, 1.0, 1.0, alpha);
}