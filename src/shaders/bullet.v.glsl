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

uniform b_BulletRotation {
  float a_rotation;
};

void main() {
  vec4 rot_pos = mat4(cos(a_rotation),  -sin(a_rotation),  0.0,   0.0,
                      sin(a_rotation),  cos(a_rotation),   0.0,   0.0,
                      0.0,              0.0,               1.0,   0.0,
                      0.0,              0.0,               0.0,   1.0) * vec4(a_Pos, 1.0);
  gl_Position = vec4(a_position, 0.0, 0.0) + rot_pos * u_Proj * u_View * u_Model;
}
