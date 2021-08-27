#version 450
// From Patricio Gonzalez Vivo: 
// https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83

// and also:
// https://stackoverflow.com/questions/21272465/glsl-shadows-with-perlin-noise 

#ifdef GL_ES
precision mediump float;
#endif

layout(set = 2, binding = 0) uniform TimeComponent_value {
    float time;
};
layout(set = 2, binding = 1) uniform NoiseColorComponent_value {
    vec3 base_color;
};
layout(set = 2, binding = 2) uniform PerlinComponent {
    float resolution;
    float first_octave;
};
layout(location = 0) in float uv;
layout(location = 1) in vec2 pos;
layout(location = 0) out vec4 o_Target;

vec4 mod289(vec4 x) {
    float value = 289.0;
    return x - floor(x * (1.0 / value)) * value;
}

vec4 permute(vec4 x) {
    return mod289(((x*34.0)+1.0)*x);
}

vec2 fade(vec2 t) {return t*t*t*(t*(t*6.0-15.0)+10.0);}

float classicPerlinNoise(vec2 P){
    vec4 Pi = floor(P.xyxy) + vec4(0.0, 0.0, 1.0, 1.0);
    vec4 Pf = fract(P.xyxy) - vec4(0.0, 0.0, 1.0, 1.0);
    Pi = mod(Pi, 289.0); // To avoid truncation effects in permutation
    vec4 ix = Pi.xzxz;
    vec4 iy = Pi.yyww;
    vec4 fx = Pf.xzxz;
    vec4 fy = Pf.yyww;
    vec4 i = permute(permute(ix) + iy);
    vec4 gx = 2.0 * fract(i * 0.0243902439) - 1.0; // 1/41 = 0.024...
    vec4 gy = abs(gx) - 0.5;
    vec4 tx = floor(gx + 0.5);
    gx = gx - tx;
    vec2 g00 = vec2(gx.x,gy.x);
    vec2 g10 = vec2(gx.y,gy.y);
    vec2 g01 = vec2(gx.z,gy.z);
    vec2 g11 = vec2(gx.w,gy.w);
    vec4 norm = 1.79284291400159 - 0.85373472095314 * 
    vec4(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 *= norm.x;
    g01 *= norm.y;
    g10 *= norm.z;
    g11 *= norm.w;
    float n00 = dot(g00, vec2(fx.x, fy.x));
    float n10 = dot(g10, vec2(fx.y, fy.y));
    float n01 = dot(g01, vec2(fx.z, fy.z));
    float n11 = dot(g11, vec2(fx.w, fy.w));
    vec2 fade_xy = fade(Pf.xy);
    vec2 n_x = mix(vec2(n00, n01), vec2(n10, n11), fade_xy.x);
    float n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}

void main()
{
    vec2 xy = pos / vec2(resolution);

float n = 0;
n+= first_octave * classicPerlinNoise(xy - vec2(time));
n+= first_octave / 2. * classicPerlinNoise(-xy * 2. - vec2(time * 1.4));
n+= first_octave / 4. * classicPerlinNoise(xy * 4. - vec2(time * 2.0));
n+= first_octave / 8.  * classicPerlinNoise(-xy * 8. - vec2(time * 2.8));
n+= first_octave / 16. * classicPerlinNoise(xy * 16. - vec2(time * 4.0));
n+= first_octave / 32. * classicPerlinNoise(-xy * 32. - vec2(time * 8.0));

    vec4 result = vec4(base_color, uv + n);

    o_Target = result;
}