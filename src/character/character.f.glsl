#version 410 core

in vec2 v_BufPos;
out vec4 Target0;

struct CharacterData {
    vec4 data;
};

const int CHARACTER_BUF_LENGTH = 512;

uniform b_Character {
  CharacterData u_Data[CHARACTER_BUF_LENGTH];
};

uniform b_PsLocals {
  vec2 u_characterSheetIdx;
  vec4 u_CharactersheetSize;
};

uniform sampler2D t_CharacterSheet;

void main() {
  int bufIdx = int(u_characterSheetIdx.x);
  vec4 entry = u_Data[bufIdx].data;
  vec2 uvCoords = (entry.xy) / u_CharactersheetSize.x;

  Target0 = texture(t_CharacterSheet, uvCoords);
}
