#version 400 core

in vec3 a_Pos;
in vec2 a_BufPos;
out vec2 v_BufPos;

//layout(location = 0) in vec2 position;

uniform b_VsLocals {
  mat4 u_Model;
  mat4 u_View;
  mat4 u_Proj;
};

uniform Bounds {
  mat4 transform;
};

void main() {
  v_BufPos = a_BufPos;
//  gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos, 1.0);
  gl_Position = transform * vec4(a_Pos, 1.0);
}
