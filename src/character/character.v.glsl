#version 410 core

in vec3 a_Pos;
in vec2 a_BufPos;
out vec2 v_BufPos;

uniform b_VsLocals {
  mat4 u_Model;
  mat4 u_View;
  mat4 u_Proj;
};

uniform t_CharacterIdx {
  float idx;
};

void main() {
  float x_sheet = 11.0;
  float y_sheet = 19.0;
  v_BufPos = vec2(a_BufPos);

  v_BufPos.x /= x_sheet;
  v_BufPos.x += (4.0 / x_sheet);
  v_BufPos.y /= y_sheet;
  v_BufPos.y += (4.0 / y_sheet);

  gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos, 1.0);
}
