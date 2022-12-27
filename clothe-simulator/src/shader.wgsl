// Vertex shader

struct CameraUniform {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
};

struct Sphere {
    x: f32,
    y: u32,
    z: f32,
    radius: f32,
}

@group(1) @binding(0) var<uniform> matrices: CameraUniform;
@group(1) @binding(0) var<uniform> data: Sphere;

struct InstanceInput {
    @location(5) translation: vec3<f32>,
    @location(6) velocity: vec3<f32>,
};

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) velocity: vec4<f32>,
    @location(3) resultant: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) norm: vec4<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = vec2<f32>(0.0, 0.0);// model.tex_coords;
    out.clip_position = matrices.proj * matrices.view * model.position;
    out.norm = model.normal;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var norm: vec3<f32> = vec3(in.norm[0], in.norm[1], in.norm[2]);
    var l: vec3<f32> = vec3(1.0);
    var v_cam = vec3(0.0, 0.0, 0.4);

    return max(dot(l, norm), 0.05) * vec4(0.5, 0.0, 0.0, 1.0);

    // if in.norm[2] > 0.5 {
    //     return vec4(0.5, 0.0, 0.0, 1.0);
    // } else {
    //     return vec4(0.0, 0.5, 0.0, 1.0);
    // }
}
