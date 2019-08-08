#version 150 core

in vec2 v_BufPos;
out vec4 Target0;

struct TileMapData {
  vec4 data;
};

const int TILEMAP_BUF_LENGTH = 4096;

layout (std140) uniform b_TileMap {
  TileMapData u_Data[TILEMAP_BUF_LENGTH];
};

layout (std140) uniform b_PsLocals {
  vec2 u_WorldSize;
  vec2 u_TilesheetSize;
};

uniform sampler2D t_TileSheet;

const vec3 lightPos = vec3(-200.0, 150.0, 0.0);
const vec3 Normal = vec3(0.0, 1.0, 0.0);
const vec3 lightColor = vec3(0.8, 0.5, 0.5);

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

  vec3 norm = normalize(Normal);
  vec3 lightDir = normalize(lightPos - vec3(v_BufPos, 0.0));

  float diff = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = diff * lightColor;

  vec4 tex = texture(t_TileSheet, uvCoords);
  tex *= vec4(diffuse, 0.0);
  Target0 = tex;
}
