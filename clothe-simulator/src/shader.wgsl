// Vertex shader

struct CameraUniform {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> matrices: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) position: vec3<f32>,
    @location(2) position: vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
    @location(4) velocity: vec3<f32>,
    @location(5) resultant: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = matrices.proj * matrices.view * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(0.5, 0.0, 0.0, 1.0);
}
