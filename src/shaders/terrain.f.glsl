#version 150 core

in vec2 v_BufPos;
out vec4 Target0;

struct TileMapData {
  vec4 data;
};

const int TILEMAP_BUF_LENGTH = 4096;
const float SHADING_MULTIPLIER = 0.2;

layout (std140) uniform b_TileMap {
  TileMapData u_Data[TILEMAP_BUF_LENGTH];
};

layout (std140) uniform b_PsLocals {
  vec2 u_WorldSize;
  vec2 u_TilesheetSize;
};

uniform sampler2D t_TileSheet;

void main() {
  vec2 bufTileCoords = floor(v_BufPos);
  vec2 rawUvOffsets = vec2(v_BufPos.x - bufTileCoords.x, 1.0 - (v_BufPos.y - bufTileCoords.y));

  int bufIdx = int((bufTileCoords.y * u_WorldSize.x) + bufTileCoords.x);
  vec4 entry = u_Data[bufIdx].data;
  vec2 coords = vec2(0.0, 0.0);

  if (bufIdx < TILEMAP_BUF_LENGTH) {
    coords = vec2(mod(entry.x, u_TilesheetSize.y), floor(entry.x / u_TilesheetSize.x));
  } else if (bufIdx < (TILEMAP_BUF_LENGTH * 2)) {
    entry = u_Data[bufIdx - TILEMAP_BUF_LENGTH].data;
    coords = vec2(mod(entry.y, u_TilesheetSize.y), floor(entry.y / u_TilesheetSize.x));
  } else if (bufIdx < (TILEMAP_BUF_LENGTH * 3)) {
    entry = u_Data[bufIdx - TILEMAP_BUF_LENGTH * 2].data;
    coords = vec2(mod(entry.z, u_TilesheetSize.y), floor(entry.z / u_TilesheetSize.x));
  } else {
    entry = u_Data[bufIdx-TILEMAP_BUF_LENGTH * 3].data;
    coords = vec2(mod(entry.w, u_TilesheetSize.y), floor(entry.w / u_TilesheetSize.x));
  }
  vec2 uvCoords = (coords.xy + rawUvOffsets) / u_TilesheetSize.xy;

  vec4 t0 = texture(t_TileSheet, uvCoords);
  t0 *= SHADING_MULTIPLIER;
  Target0 = t0;
}
