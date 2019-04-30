#version 450

layout(location = 0) in vec2 v_TexCoord;
layout(location = 0) out vec4 Target0;
layout(set = 0, binding = 1) uniform texture2D t_Color;
layout(set = 0, binding = 2) uniform sampler t_CharacterSheet;

const float SHADING_MULTIPLIER = 0.5;

void main() {
  vec4 tex = texture(sampler2D(t_Color, t_CharacterSheet), v_TexCoord);
  tex *= SHADING_MULTIPLIER;
  if (tex.a < 0.1) {
    discard;
  }
  Target0 = tex;
}
