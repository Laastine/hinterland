#version 450

layout(location = 0) in vec4 a_Pos;
layout(location = 1) in vec2 a_TexCoord;
layout(location = 0) out vec2 v_TexCoord;

layout(set = 0, binding = 0) uniform Locals {
  mat4 u_Model;
  mat4 u_View;
  mat4 u_Proj;
};

layout(set = 0, binding = 4) uniform b_TileMapPosition {
  vec2 a_position;
};

void main() {
  v_TexCoord = a_TexCoord;
  gl_Position = vec4(a_position, 0.0, 0.0) + u_Proj * u_View * u_Model * a_Pos;
  // convert from -1,1 Z to 0,1
  gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
}
