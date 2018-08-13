#version 330 core

in vec3 a_Pos;
in vec2 a_BufPos;
out vec2 v_BufPos;

uniform b_TextPosition {
  vec2 a_position;
};

void main() {
  v_BufPos = a_BufPos * vec2(25.0, 50.0);
  gl_Position = vec4(a_position, 0.0, 0.0) + vec4(a_Pos, 1.0);
}
