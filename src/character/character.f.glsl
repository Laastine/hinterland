#version 410 core

in vec2 v_BufPos;
out vec4 Target0;

const int CHARACTER_BUF_LENGTH = 210;

uniform sampler2D t_CharacterSheet;

void main() {
  vec4 tex = texture(t_CharacterSheet, v_BufPos).rgba;
  if(tex.a < 0.1) {
    discard;
  }
  Target0 = tex;
}
