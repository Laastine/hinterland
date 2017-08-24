#version 410 core

in vec2 v_BufPos;
out vec4 Target0;

struct CharacterData {
    vec4 data;
};

const int CHARACTER_BUF_LENGTH = 210;

uniform b_Character {
  CharacterData u_Data[CHARACTER_BUF_LENGTH];
};

uniform b_PsLocals {
  vec4 u_CharacterSize;
  vec4 u_CharacterSheetSize;
  vec2 u_CharacterOffsets;
};

uniform sampler2D t_CharacterSheet;

void main() {
  vec4 tex = texture(t_CharacterSheet, v_BufPos).rgba;
  if(tex.a < 0.1) {
    discard;
  }
  Target0 = tex;
}
