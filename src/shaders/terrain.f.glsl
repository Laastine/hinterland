#version 330 core

in vec2 v_BufPos;
out vec4 Target0;

struct TileMapData {
  vec4 data;
};

const int TILEMAP_BUF_LENGTH = 2304;
const float SHADING_MULTIPLIER = 0.5;

uniform b_TileMap {
  TileMapData u_Data[TILEMAP_BUF_LENGTH];
};

uniform b_PsLocals {
  vec2 u_WorldSize;
  vec2 u_TilesheetSize;
};

uniform sampler2D t_TileSheet;

void main() {
  vec2 bufTileCoords = floor(v_BufPos);
  vec2 rawUvOffsets = vec2(v_BufPos.x - bufTileCoords.x, 1.0 - (v_BufPos.y - bufTileCoords.y));

  int bufIdx = int((bufTileCoords.y * u_WorldSize.x) + bufTileCoords.x);
  vec4 entry = u_Data[bufIdx].data;
  vec2 uvCoords = (entry.xy + rawUvOffsets) / u_TilesheetSize.xy;

  vec4 t0 = texture(t_TileSheet, uvCoords);
  t0 *= SHADING_MULTIPLIER;
  Target0 = t0;
}
