#version 150 core

in vec2 v_BufPos;
out vec4 Target0;

uniform sampler2D t_CharacterSheet;

const float SHADING_MULTIPLIER = 0.5;

void main() {
  vec4 tex = texture(t_CharacterSheet, v_BufPos).rgba;
  tex *= SHADING_MULTIPLIER;
  if(tex.a < 0.1) {
    discard;
  }
  Target0 = tex;
}
