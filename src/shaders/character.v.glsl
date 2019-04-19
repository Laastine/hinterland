#version 450

layout(location = 0) in vec4 a_Pos;
layout(location = 1) in vec2 a_TexCoord;
layout(location = 0) out vec2 v_TexCoord;

layout(set = 0, binding = 0) uniform Locals {
  mat4 u_Model;
  mat4 u_View;
  mat4 u_Proj;
};

layout(set = 0, binding = 3) uniform b_CharacterSprite {
  float x_div;
  float y_div;
  int a_row;
  int a_index;
};

layout(set = 0, binding = 4) uniform b_CharacterPosition {
  vec2 a_position;
};

void main() {
  v_TexCoord = vec2(a_TexCoord);

  v_TexCoord.y += y_div;
  if (a_row > 1) {
    v_TexCoord.y /= 2.0;
  }
  v_TexCoord.x /= x_div;
  v_TexCoord.x += a_index / x_div;

  gl_Position = vec4(a_position, -1.0, 0.0) + u_Proj * u_View * u_Model * a_Pos;
  // convert from -1,1 Z to 0,1
  gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
}
