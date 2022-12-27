struct Vertex {
    position: vec4<f32>,
    normal: vec4<f32>,
    velocity: vec4<f32>,
    resultant: vec4<f32>
}

struct ComputeData {
    spring_contant: f32,
    damping_factor: f32,
    gravity: f32,
    delta_time: f32,
}

@group(0) @binding(0) var<storage, read_write> vertices: array<Vertex>;

@compute @workgroup_size(64, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    if (param.x > u32(25)) {
        return;
    }

    vertices[param.x].resultant[0] = 0.0;
    vertices[param.x].resultant[1] = 0.0;
    vertices[param.x].resultant[2] = 0.0;
}