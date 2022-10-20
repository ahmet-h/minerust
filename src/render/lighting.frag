#version 330 core

in vec2 tex_coords;

uniform sampler2D g_position;
uniform sampler2D g_normal;
uniform sampler2D g_albedo_spec;

uniform vec3 view_pos;

out vec4 color;

void main() {
    vec3 position = texture(g_position, tex_coords).rgb;
    vec3 normal = texture(g_normal, tex_coords).rgb;
    vec3 albedo = texture(g_albedo_spec, tex_coords).rgb;
    float specular_strength = texture(g_albedo_spec, tex_coords).a;

    vec3 light_color = vec3(1.0, 1.0, 1.0);

    vec3 lighting = albedo * 0.1;
    vec3 view_dir = normalize(view_pos - position);

    vec3 light_dir = normalize(vec3(0.5, -1.0, -0.8));
    vec3 diffuse = max(dot(-light_dir, normal), 0.0) * albedo * light_color;

    vec3 reflect_dir = reflect(light_dir, normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 8.0);
    vec3 specular = light_color * spec * specular_strength;

    lighting += diffuse + specular;

    color = vec4(lighting, 1.0);
}