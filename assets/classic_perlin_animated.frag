#version 450
// From Patricio Gonzalez Vivo: 
// https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83

// and also:
// https://stackoverflow.com/questions/21272465/glsl-shadows-with-perlin-noise 

#ifdef GL_ES
precision mediump float;
#endif

//uniform vec2 u_resolution;
layout(set = 2, binding = 0) uniform TimeComponent_value {
    float time;
};
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
    //vec2 uv = gl_FragCoord.xy / u_resolution.xy;
    vec2 uv = gl_FragCoord.xy / vec2(1000., 1000.);
    //uv.x *= u_resolution.x / u_resolution.y;

    float scale = 8.;
    uv *= scale;

    float noise = classicPerlinNoise(uv);
    vec3 animated_noise = vec3(0.2*sin(time + 6.2831 * noise));

    vec3 base_color = vec3(0.5, 0.5, 0.0);
    animated_noise += base_color;

    o_Target = vec4(animated_noise, 0.2);
}