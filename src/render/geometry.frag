#version 330 core

layout (location = 0) out vec3 g_position;
layout (location = 1) out vec3 g_normal;
layout (location = 2) out vec4 g_albedo_spec;

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform sampler2D texture_diffuse;

void main() {
    g_position = position;
    g_normal = normal;
    g_albedo_spec.rgb = texture(texture_diffuse, tex_coords).rgb;
    g_albedo_spec.a = 0.4;
}