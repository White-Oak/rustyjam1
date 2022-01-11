#version 450
layout(location = 0) in float uv;
//layout(location = 1) in vec3 v_Color;
    // 0 to 1
layout(set = 2, binding = 0) uniform PercentComponent_value {
    float percent;
};
layout(location = 0) out vec4 o_Target;

void main() {
    if (uv >= percent) {
        o_Target = vec4(0.196, 0.471, 0.878, 1.0);
    } else {
        o_Target = vec4(0.196, 0.471, 0.878, 0.0);
    }
}