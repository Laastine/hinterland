#version 410 core

in vec3 a_Pos;
in vec2 a_BufPos;
out vec2 v_BufPos;

void main() {
  v_BufPos = a_BufPos;
  gl_Position = vec4(a_Pos, 1.0);
}
