#version 330 core

in vec3 cam_dir;

out vec4 color;

void main() {
    // vec3 a = mix(vec3(0.6, 0.9, 1.0), vec3(0.1, 0.4, 1.0), clamp(cam_dir.y, 0.0, 1.0));
    // vec3 a = mix(vec3(0.0), vec3(1.0), clamp(cam_dir.y, 0.0, 1.0));

    color = vec4(cam_dir, 1.0);
}