#version 330 core

layout (location = 0) in vec3 pos;

out vec3 cam_dir;

uniform mat4 inv_view_projection;
uniform mat4 projection_view;

void main() {
    cam_dir = vec3(inv_view_projection * vec4(pos, 1.0));
    gl_Position = projection_view * vec4(pos, 1.0);
}