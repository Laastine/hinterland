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
   float a_div;
   float a_index;
};

uniform b_CharacterPosition {
  vec2 a_position;
};

void main() {
  v_BufPos = vec2(a_BufPos);

  v_BufPos.x /= a_div;
  v_BufPos.x += a_index / a_div;

  gl_Position = vec4(a_position, 0.0, 0.0) +  u_Proj * u_View * u_Model * vec4(a_Pos, 1.0);
}
