#version 330 core

layout (location = 0) in vec3 in_position;

uniform mat4 projection_view;
uniform mat4 model;

void main()
{
    gl_Position = projection_view * model * vec4(in_position, 1.0);
}