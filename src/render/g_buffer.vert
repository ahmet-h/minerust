#version 330 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_normal;
layout (location = 2) in vec2 in_tex_coords;

out vec3 position;
out vec3 normal;
out vec2 tex_coords;

uniform mat4 model;
uniform mat4 projection_view;

void main() {
    vec4 world_pos = model * vec4(in_position, 1.0);
    position = world_pos.xyz;
    tex_coords = in_tex_coords;

    mat3 normal_matrix = transpose(inverse(mat3(model)));
    normal = normalize(normal_matrix * in_normal);

    gl_Position = projection_view * world_pos;
}