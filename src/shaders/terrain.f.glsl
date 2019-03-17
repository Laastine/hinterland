#version 450

layout(location = 0) in vec2 v_TexCoord;
layout(location = 0) out vec4 Target0;
layout(set = 0, binding = 1) uniform texture2D t_Color;
layout(set = 0, binding = 2) uniform sampler t_TileSheet;

struct TileMapData {
  vec4 data;
};

const int TILEMAP_BUF_LENGTH = 4096;
const float SHADING_MULTIPLIER = 0.2;

const vec2 u_WorldSize = vec2(128.0, 128.0);
const vec2 u_TilesheetSize = vec2(32.0, 32.0);

layout (set = 0, binding = 3) uniform b_TileMap {
  TileMapData u_Data[TILEMAP_BUF_LENGTH];
};

void main() {
  vec2 bufTileCoords = floor(v_TexCoord);
  vec2 rawUvOffsets = vec2(v_TexCoord.x - bufTileCoords.x, 1.0 - (v_TexCoord.y - bufTileCoords.y));

  int bufIdx = int((bufTileCoords.y * u_WorldSize.x) + bufTileCoords.x);
  vec4 entry = u_Data[bufIdx].data;
  vec2 coords = vec2(0.0, 0.0);

  if (bufIdx < TILEMAP_BUF_LENGTH) {
    coords = vec2(mod(entry.x, u_TilesheetSize.y), floor(entry.x / u_TilesheetSize.x));
  } else if (bufIdx < (TILEMAP_BUF_LENGTH * 2)) {
    entry = u_Data[bufIdx - TILEMAP_BUF_LENGTH].data;
    coords = vec2(mod(entry.y, u_TilesheetSize.y), floor(entry.y / u_TilesheetSize.x));
  } else if (bufIdx < (TILEMAP_BUF_LENGTH * 3)) {
    entry = u_Data[bufIdx - TILEMAP_BUF_LENGTH*2].data;
    coords = vec2(mod(entry.z, u_TilesheetSize.y), floor(entry.z / u_TilesheetSize.x));
  } else {
    entry = u_Data[bufIdx-TILEMAP_BUF_LENGTH*3].data;
    coords = vec2(mod(entry.w, u_TilesheetSize.y), floor(entry.w / u_TilesheetSize.x));
  }
  vec2 uvCoords = (coords.xy + rawUvOffsets) / u_TilesheetSize.xy;

  vec4 t0 = texture(sampler2D(t_Color, t_TileSheet), uvCoords);
  t0 *= SHADING_MULTIPLIER;
  Target0 = t0;
}
