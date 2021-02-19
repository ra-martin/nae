#version 450
precision highp float;

layout(location = 0) out vec4 outColor;
layout(location = 0) in vec2 v_texcoord;

layout(location = 0) uniform sampler2D u_texture;
layout(location = 1) uniform vec2 u_size;
layout(location = 2) uniform float u_delta;
layout(location = 3) uniform vec2 u_brush;

// Adapted from https://gist.github.com/sansumbrella/72c15b21ba2062c19c5943ee3e4574a0#file-simulate-fs

void main()
{
    vec2 uv = v_texcoord;
    vec2 pixel_size = vec2(1.0, 1.0) / u_size;

    vec2 ab = texture(u_texture, uv).xy;
    vec2 laplace = vec2(0);

    laplace += texture(u_texture, uv + pixel_size * vec2(-1, -1)).xy * 0.05;
    laplace += texture(u_texture, uv + pixel_size * vec2(0, -1)).xy * 0.2;
    laplace += texture(u_texture, uv + pixel_size * vec2(1, -1)).xy * 0.05;
    laplace += texture(u_texture, uv + pixel_size * vec2(-1, 0)).xy * 0.2;
    laplace += ab * (-1.0);
    laplace += texture(u_texture, uv + pixel_size * vec2(1, 0)).xy * 0.2;
    laplace += texture(u_texture, uv + pixel_size * vec2(-1, 1)).xy * 0.05;
    laplace += texture(u_texture, uv + pixel_size * vec2(0, 1)).xy * 0.2;
    laplace += texture(u_texture, uv + pixel_size * vec2(1, 1)).xy * 0.05;

    // Known good values
    float diffusion_a = 0.2097; // mix(0.9, 1.03, uv.x);
    float diffusion_b = 0.105; // mix(0.4, 0.58, uv.y);
    float feed = 0.037; // mix(0.05, 0.06, uv.x);
    float kill = 0.06; // mix(0.060, 0.064, uv.y);
    float dt = 1.0 + (u_delta * 0.0000001); // avoid optimization

    float a = ab.x;
    float b = ab.y;

    float ap = clamp(a + ((diffusion_a * laplace.x) - (a * b * b) + (feed * (1.0 - a))) * dt, 0, 1);
    float bp = clamp(b + ((diffusion_b * laplace.y) + (a * b * b) - ((kill + feed) * b)) * dt, 0, 1);

    if(u_brush.x > 0.0)
    {
        if(distance(v_texcoord, u_brush) < 0.025) {
            bp = 0.90;
        }
    }

    outColor = vec4(ap, bp, 0, 1);

}