#version 450

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ShaderInputs_time {
    float time;
};

layout(set = 2, binding = 1) uniform ShaderInputs_resolution {
    vec2 resolution;
};

vec3 palette(float d) {
    return mix(vec3(0.74, 0.14, 0.35), vec3(0.9, 0.0, 1.0), d);
}

vec2 rotate(vec2 p, float a) {
    float c = cos(a);
    float s = sin(a);
    return p*mat2(c, s, -s, c);
}

float map(vec3 p) {
    vec3 q = p;
    for (int i = 0; i < 8; ++i) {
        float t = time * 0.2;          // Animation speed
        q.xz = rotate(q.xz, t * 1.1);  // Shape morphing
        q.xy = rotate(q.xy, t * 1.89); // Shape folding
        q.xz = abs(q.xz);              // Get multiple points
        q.xz -= 0.5;                   // Zoom. Larger values --> More zoomed.
    }
    return dot(sign(q), q) / 5.0; // Controls glow of points. Divide by larger values for more glow.
}

vec4 raymarch(vec3 ray_origin, vec3 ray_direction) {
    float t = 0.0;
    vec3 col = vec3(0.0);
    float d;

    for (float i = 0.0; i < 64.0; i++) {
        vec3 p = ray_origin + ray_direction*t;
        d = map(p) * 0.5;

        // Clip d between 0.02 and 100.0. Affects what gets rendered.
        if (d < 0.02) {
            break;
        }
        if (d > 2000.0) {
            break;
        }

        // col += vec3(0.6,0.8,0.8)/(400.*(d));
        col += palette(length(p) * 0.1) / (400.0 * d);
        t += d;
    }

    return vec4(col, 0.01 / d);
}

void main() {
    vec2 uv = resolution * (v_Uv - 0.5) / resolution.x;
    vec3 ray_origin = vec3(0.0, 0.0, -50.0);
    ray_origin.xz = rotate(ray_origin.xz, time);

    vec3 cf = normalize(-ray_origin);
    vec3 cs = normalize(cross(cf, vec3(0.0, 1.0, 0.0)));
    vec3 cu = normalize(cross(cf, cs));

    vec3 uuv = ray_origin + cf*3.0 + uv.x*cs + uv.y*cu;

    vec3 ray_direction = normalize(uuv - ray_origin);

    vec4 col = raymarch(ray_origin, ray_direction);

    o_Target = col * 1.5;
}
