#version 300 es
precision mediump float;

in vec3 vertexPosition;
uniform mat4 vertexTransform;
out vec3 pos;

void main() {
    gl_Position = vec4(vertexPosition, 1.0) * vertexTransform;
    pos = vertexPosition;
}
