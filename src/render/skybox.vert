#version 330 core

layout (location = 0) in vec3 in_position;

out vec3 tex_coords;

uniform mat4 projection_view;

void main() {
    tex_coords = in_position;
    vec4 world_pos = projection_view * vec4(in_position, 1.0);
    gl_Position = world_pos.xyww;
}