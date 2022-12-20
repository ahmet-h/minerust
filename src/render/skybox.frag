#version 330 core

in vec3 tex_coords;

out vec4 color;

uniform samplerCube skybox;

void main() {
    color = texture(skybox, tex_coords);

    vec3 lighting = color.rgb;

    lighting = lighting / (lighting + vec3(1.0));
    lighting = pow(lighting, vec3(1.0 / 2.2));

    color = vec4(lighting, 1.0);
}