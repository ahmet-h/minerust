#version 330 core

in vec2 tex_coords;

uniform sampler2D g_position;
uniform sampler2D g_normal;
uniform sampler2D g_albedo_spec;

out vec4 color;

void main() {
    vec3 position = texture(g_position, tex_coords).rgb;
    vec3 normal = texture(g_normal, tex_coords).rgb;
    vec3 diffuse = texture(g_albedo_spec, tex_coords).rgb;
    float specular = texture(g_albedo_spec, tex_coords).a;

    vec3 lighting = diffuse * 0.2;

    color = vec4(lighting, 1.0);
}