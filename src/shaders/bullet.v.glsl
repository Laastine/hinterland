#version 150 core

in vec3 a_Pos;

uniform b_VsLocals {
  mat4 u_Model;
  mat4 u_View;
  mat4 u_Proj;
};

uniform b_BulletPosition {
  vec2 a_position;
};

void main() {
  gl_Position = vec4(a_position, 0.0, 0.0) + u_Proj * u_View * u_Model * vec4(a_Pos, 1.0);
}
