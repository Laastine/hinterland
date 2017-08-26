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
   vec2 a_div;
   vec2 a_index;
};

void main() {
  float x_sheet = 11.0;
  float y_sheet = 19.0;
  v_BufPos = vec2(a_BufPos);

  v_BufPos.x /= a_div.x;
  v_BufPos.x += (a_index.x / a_div.x);
  v_BufPos.y /= a_div.y;
  v_BufPos.y += (a_index.y / a_div.y);

  gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos, 1.0);
}
