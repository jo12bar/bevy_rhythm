#version 450

#define TWO_PI 6.28318530718

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ShaderInputs_time {
    float time;
};
layout(set = 2, binding = 1) uniform ShaderInputs_resolution {
    vec2 resolution;
};
layout(set = 3, binding = 0) uniform TimeSinceLastCorrect_last_time {
    float last_time;
};
layout(set = 3, binding = 1) uniform TimeSinceLastCorrect_points {
    float points;
};

float interval(in float a, in float b, in float val) {
    return step(a, val) * smoothstep(0.9 - b, 1.0 - b, 1.0 - val);
}

float circle(in vec2 uv, in float _radius) {
    vec2 dist = uv - vec2(0.5);
    return 1.0 - smoothstep(0.99 * _radius,
                            1.01 * _radius,
                            dot(dist, dist) * 4.0);
}

float smoothcircle(in vec2 _st, in float s) {
    vec2 dist = _st - vec2(0.5);
    return 4.0 * dot(dist, dist) / s;
}

void main() {
    // == 0.0 when the circle isn't shown:
    float alpha = interval(last_time, last_time + 0.6, time);

    // Circle radius
    float radius = time - last_time;

    //   == 0.0 when not in the circle
    //   == 1.0 when in the circle
    // float circle = circle(v_Uv, radius) * (1. - circle(v_Uv, radius - 0.1));
    float circle = smoothcircle(v_Uv, radius) * smoothcircle(v_Uv, radius) * circle(v_Uv, radius);

    // rgb(230, 153, 0);
    vec3 colorMin = vec3(0.901960784, 0.6, 0.0);
    // rgb(86, 200, 0);
    vec3 colorMax = vec3(0.337254902, 0.784313725, 0.0);

    // Get color according to points
    vec3 color = mix(colorMin, colorMax, points);

    o_Target = vec4(color * circle, circle * alpha);
}
