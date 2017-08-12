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
  vec4 u_CharacterSize;
  vec4 u_CharacterSheetSize;
  vec2 u_CharacterOffsets;
};

uniform sampler2D t_CharacterSheet;

void main() {
  vec2 offset_bufpos = v_BufPos + (u_CharacterOffsets / u_CharacterSize.zz);
  vec2 bufTileCoords = floor(offset_bufpos);
  vec2 rawUvOffsets = vec2(offset_bufpos.x - bufTileCoords.x, 1.0 - (offset_bufpos.y - bufTileCoords.y));

  int bufIdx = int((bufTileCoords.y * u_CharacterSize.x) + bufTileCoords.x);
  vec4 entry = u_Data[bufIdx].data;
  vec2 uvCoords = (entry.xy + rawUvOffsets) / u_CharacterSheetSize.xy;

  vec4 tex = texture(t_CharacterSheet, uvCoords).rgba;
  if(tex.a < 0.1) {
    discard;
  }
  Target0 = tex;
}
