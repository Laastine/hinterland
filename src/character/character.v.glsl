#version 410 core

in vec3 a_Pos;
in vec2 a_BufPos;
out vec2 v_BufPos;

uniform b_VsLocals {
  mat4 transform;
};

uniform t_CharacterIdx {
  float idx;
};

void main() {
  v_BufPos = a_BufPos * vec2(0.1, -0.1) * vec2((1.0 / 11.0), (1.0 / 19.0));
  gl_Position = transform * vec4(a_Pos, 1.0);
}
