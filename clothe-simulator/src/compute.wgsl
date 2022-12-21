struct Vertex {
    position: vec4<f32>,
    normal: vec4<f32>,
    velocity: vec4<f32>,
    resultant: vec4<f32>
}

struct Sphere {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    spring_contant: f32,
    damping_factor: f32,
    gravity: f32,
    delta_time: f32,
}

@group(0) @binding(0) var<storage, read_write> vertices: array<Vertex>;
@group(1) @binding(0) var<uniform> data: Sphere;


@compute @workgroup_size(64, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    /*if (param.x >= u32(data.nb_instances)) {
          return;
    }*/

    var vertex = vertices[param.x];

    // Add gravity
    // vertices[param.x].velocity[2] += data.gravity * data.delta_time;

    vertices[param.x].position[0] += vertices[param.x].velocity[0] * data.delta_time;
    vertices[param.x].position[1] += vertices[param.x].velocity[1] * data.delta_time;
    vertices[param.x].position[2] += vertices[param.x].velocity[2] * data.delta_time;
}