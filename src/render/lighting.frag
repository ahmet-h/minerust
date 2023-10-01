#version 330 core

in vec2 tex_coords;

uniform sampler2D g_position;
uniform sampler2D g_normal;
uniform sampler2D g_albedo_spec;
uniform sampler2DShadow shadow_map;

uniform vec3 view_pos;
uniform mat4 shadow_projection_view;
uniform vec3 light_dir;

out vec4 color;

#define EPSILON 0.00001
#define SHADOW_WIDTH 4096.0
#define SHADOW_HEIGHT 4096.0

#define SHADOW_BLUR 1
#define SHADOW_BLUR_WIDTH (SHADOW_BLUR * 2 + 1)
#define SHADOW_FACTOR_SIZE (SHADOW_BLUR_WIDTH * SHADOW_BLUR_WIDTH)

void main() {
    vec3 position = texture(g_position, tex_coords).rgb;
    vec3 normal = texture(g_normal, tex_coords).rgb;
    vec3 albedo = texture(g_albedo_spec, tex_coords).rgb;
    float specular_strength = texture(g_albedo_spec, tex_coords).a;
    vec4 light_view_position = shadow_projection_view * vec4(position, 1.0);

    vec3 light_color = vec3(1.0, 1.0, 1.0);

    vec3 lighting = albedo * 0.2;
    vec3 view_dir = normalize(view_pos - position);

    vec3 diffuse = max(dot(-light_dir, normal), 0.0) * albedo * light_color;

    vec3 reflect_dir = reflect(light_dir, normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 8.0);
    vec3 specular = light_color * spec * specular_strength;

    vec2 shadow_step = vec2(1.0 / SHADOW_WIDTH, 1.0 / SHADOW_HEIGHT);
    float shadow_factor = 0.0;
    for (int y = -SHADOW_BLUR; y <= SHADOW_BLUR; y++) {
        for (int x = -SHADOW_BLUR; x <= SHADOW_BLUR; x++) {
            vec2 offsets = vec2(x * shadow_step.x, y * shadow_step.y);
            vec3 uvc = vec3(light_view_position.xy + offsets, light_view_position.z + EPSILON);
            shadow_factor += texture(shadow_map, uvc);
        }
    }

    lighting += diffuse * (0.5 + (shadow_factor / (float(SHADOW_FACTOR_SIZE) * 2))) +
        specular * (shadow_factor / float(SHADOW_FACTOR_SIZE));

    // lighting = lighting / (lighting + vec3(1.0));
    float exposure = 1.0;
    lighting = vec3(1.0) - exp(-lighting * exposure);
    lighting = pow(lighting, vec3(1.0 / 2.2));

    color = vec4(lighting, 1.0);
}