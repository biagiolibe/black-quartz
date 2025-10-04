@vertex
fn vertex_main(
    @location(0) position: vec3<f32>,
    @builtin(position) out_position: ptr<function, vec4<f32>>
) -> @location(0) vec2<f32> {
    *out_position = vec4<f32>(position, 1.0);
    // Coordinate normalize da -1 a 1
    return position.xy;
}

struct Uniforms {
    alpha: f32,
};

@group(1) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fragment_main(
    @location(0) uv: vec2<f32>
) -> @location(0) vec4<f32> {
    let dist = length(uv);
    let fade = clamp(1.0 - dist, 0.0, 1.0);
    let alpha = fade * uniforms.alpha;
    return vec4<f32>(1.0, 1.0, 1.0, alpha);
}
