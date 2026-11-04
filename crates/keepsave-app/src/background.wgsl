// Slowly drifting, heavily blurred color blobs. Panels drawn on top are
// translucent, so this is what makes them read as glass.

struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    // 1.0 when the target is an sRGB format and expects linear output.
    srgb_target: f32,
};

@group(0) @binding(0) var<uniform> u: Uniforms;

// One triangle that covers the whole viewport.
@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let uv = vec2<f32>(f32((index << 1u) & 2u), f32(index & 2u));
    return vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
}

fn blob(p: vec2<f32>, center: vec2<f32>, radius: f32) -> f32 {
    let d = p - center;
    return exp(-dot(d, d) / (radius * radius));
}

// Cheap hash for dithering, so the smooth gradients don't band.
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let aspect = u.resolution.x / max(u.resolution.y, 1.0);
    var p = position.xy / u.resolution;
    p.x *= aspect;
    let t = u.time * 0.06;

    var color = vec3<f32>(0.035, 0.039, 0.063);
    color += vec3<f32>(0.30, 0.30, 0.95) * 0.55 * blob(p,
        vec2<f32>(aspect * (0.22 + 0.12 * sin(t * 1.3)), 0.28 + 0.14 * cos(t * 1.1)), 0.42);
    color += vec3<f32>(0.08, 0.62, 0.74) * 0.40 * blob(p,
        vec2<f32>(aspect * (0.80 + 0.10 * cos(t * 0.9)), 0.22 + 0.16 * sin(t * 1.4)), 0.38);
    color += vec3<f32>(0.62, 0.22, 0.86) * 0.42 * blob(p,
        vec2<f32>(aspect * (0.68 + 0.14 * sin(t * 0.7 + 1.7)), 0.86 + 0.10 * cos(t * 1.2)), 0.46);
    color += vec3<f32>(0.12, 0.30, 0.88) * 0.35 * blob(p,
        vec2<f32>(aspect * (0.18 + 0.10 * cos(t * 1.6 + 0.4)), 0.92 + 0.08 * sin(t)), 0.36);

    // Darken the corners so the center, where the cards sit, glows.
    let centered = position.xy / u.resolution - 0.5;
    color *= 1.0 - 0.55 * dot(centered, centered);

    color += (hash(position.xy) - 0.5) / 255.0;
    if u.srgb_target > 0.5 {
        color = pow(max(color, vec3<f32>(0.0)), vec3<f32>(2.2));
    }
    return vec4<f32>(color, 1.0);
}
