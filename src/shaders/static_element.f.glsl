#version 150 core

in vec2 v_BufPos;
out vec4 Target0;

uniform sampler2D t_StaticElementSheet;

uniform b_TimeModulo {
  float a_time;
};

const float PI = 3.1415926535897932384626433832795;
const vec3 lightOrigPos = vec3(-200.0, 150.0, 0.0);
const vec3 Normal = vec3(0.0, 1.0, 0.0);
const vec3 lightColor = vec3(0.8, 0.5, 0.5);
const vec3 ambientColor = vec3(0.2, 0.2, 0.2);

void main() {
  float lightAngle = (a_time + 1) * 4;
  float lightAngleRad = lightAngle * PI / 180.0;

  vec3 lightPos = mat3(cos(lightAngleRad),  -sin(lightAngleRad),  0.0,
                        sin(lightAngleRad),  cos(lightAngleRad),  0.0,
                        0.0,       0.0,       1.0) * lightOrigPos;

  vec3 norm = normalize(Normal);
  vec3 lightDir = normalize(lightPos - vec3(v_BufPos, 0.0));

  float diff = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = diff * lightColor;

  vec4 tex = texture(t_StaticElementSheet, v_BufPos).rgba;
  tex *= vec4(diffuse + ambientColor, 1.0);
  if(tex.a < 0.1) {
    discard;
  }
  Target0 = tex;
}
