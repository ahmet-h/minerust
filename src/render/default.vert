#version 330 core

in vec3 in_position;
out vec3 position;

void main() {
    position = in_position;
    gl_Position = vec4(position, 1.0);
}