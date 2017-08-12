#version 410 core

in vec3 a_Pos;
in vec2 a_BufPos;
out vec2 v_BufPos;

uniform t_CharacterIdx {
  float idx;
};

uniform b_VsLocals {
  mat4 u_Model;
  mat4 u_View;
  mat4 u_Proj;
};

void main() {
  vec2 scale_down = vec2(0.3, -0.3);
  vec4 translate_y = vec4(0.0, 500.0, 0.0, 0.0);
  v_BufPos = a_BufPos * scale_down * vec2((1.0 / 11.0), (1.0 / 19.0));
  gl_Position = translate_y + u_Proj * u_View * u_Model * vec4(a_Pos, 1.0);
}
