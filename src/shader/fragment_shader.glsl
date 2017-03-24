#version 150 core

in vec2 v_BufPos;
out vec4 Target0;

struct TileMapData {
    vec4 data;
};

const int TILEMAP_BUF_LENGTH = 2304;

uniform b_TileMap {
    TileMapData u_Data[TILEMAP_BUF_LENGTH];
};

uniform b_PsLocals {
    vec4 u_WorldSize;
    vec4 u_TilesheetSize;
    vec2 u_TileOffsets;
};

uniform sampler2D t_TileSheet;

void main() {
    vec2 offset_bufpos = v_BufPos + (u_TileOffsets / u_WorldSize.zz);
    vec2 bufTileCoords = floor(offset_bufpos);
    vec2 rawUvOffsets = vec2(offset_bufpos.x - bufTileCoords.x, 1.0 - (offset_bufpos.y - bufTileCoords.y));

    int bufIdx = int((bufTileCoords.y * u_WorldSize.x) + bufTileCoords.x);
    vec4 entry = u_Data[bufIdx].data;
    vec2 uvCoords = (entry.xy + rawUvOffsets) / u_TilesheetSize.xy;

    Target0 = texture(t_TileSheet, uvCoords);
}
