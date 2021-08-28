#version 450
layout(location = 1) in vec3 v_Color;
layout(set = 2, binding = 0) uniform TimeComponent_value {
    float time;
};
layout(location = 0) out vec4 o_Target;
void main() {
    vec3 res = mix(v_Color, vec3(1.0, 1.0, 1.0), 0.5 + 0.5 * sin(time * 2));
    o_Target = vec4(res, 1.0);
}