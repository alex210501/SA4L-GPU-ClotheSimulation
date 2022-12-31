// Vertex shader

struct CameraUniform {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
};

@group(1) @binding(0) var<uniform> matrices: CameraUniform;

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
    out.tex_coords =  vec2(model.position[0], model.position[1]);
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
    var l: vec3<f32> = vec3(0.5);
    var v_cam = vec3(1.0, -0.5, -1.0);
    var color_light = vec4(0.1, 0.1, 0.1, 0.0);
    var r = 2.0 * dot(norm, l) * norm - l;
    var alpha = 8.0;
    var spec = pow(dot(r, v_cam), alpha) * color_light;

    return max(dot(l, norm), 0.05) * vec4(0.5, 0.0, 0.0, 1.0) + spec;
}
