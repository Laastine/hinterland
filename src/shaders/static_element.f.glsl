#version 150 core

in vec2 v_BufPos;
out vec4 Target0;

uniform sampler2D t_StaticElementSheet;

void main() {
  vec4 tex = texture(t_StaticElementSheet, v_BufPos).rgba;
  if(tex.a < 0.1) {
    discard;
  }
  Target0 = tex;
}
