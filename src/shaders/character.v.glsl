#version 410 core

in vec3 a_Pos;
in vec2 a_BufPos;
out vec2 v_BufPos;

uniform b_VsLocals {
  mat4 u_Model;
  mat4 u_View;
  mat4 u_Proj;
};

uniform b_CharacterSprite {
  float x_div;
  float y_div;
  int a_row;
  float a_index;
};

uniform b_CharacterPosition {
  vec2 a_position;
};

void main() {
  v_BufPos = vec2(a_BufPos);

  v_BufPos.y += y_div;
  if (a_row > 1) {
    v_BufPos.y /= 2.0;
  }
  v_BufPos.x /= x_div;
  v_BufPos.x += a_index / x_div;

  gl_Position = vec4(a_position, 0.0, 0.0) + u_Proj * u_View * u_Model * vec4(a_Pos, 1.0);
}
