#version 450

layout (location = 0) in vec4 fs_pos;
layout (location = 1) in vec2 fs_uv;

layout (location = 0) out vec4 out_col;

layout (binding = 0) uniform UBO {
    mat4 proj;

    uvec2 res;
    vec2 mouse;

    uint time;
} ubo;

vec3 heat(in float x) { return sin(clamp(x, 0.0, 1.0) * 3.0 - vec3(1, 2, 3)) * 0.5 + 0.5; }

void main() {
    vec3 col = vec3(0.5) + 0.5 * cos(vec3(ubo.time) / 1000.0 + fs_uv.xyx + vec3(0,2,4));
    out_col = vec4(col, 1.0);
}
