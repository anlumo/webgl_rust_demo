#version 300 es
precision mediump float;

in vec3 pos;
out vec4 fragColor;

void main() {
    fragColor = vec4(pos * 0.5 + 0.5, 1.0);
}
