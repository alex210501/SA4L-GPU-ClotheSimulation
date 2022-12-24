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
}

struct ComputeData {
    spring_contant: f32,
    damping_factor: f32,
    gravity: f32,
    delta_time: f32,
}

@group(0) @binding(0) var<storage, read_write> vertices: array<Vertex>;
@group(1) @binding(0) var<uniform> sphere: Sphere;
@group(1) @binding(1) var<uniform> data: ComputeData;


@compute @workgroup_size(64, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    if (param.x > u32(81)) {
        // vertices[0].velocity[2] = -10.0;
        return;
    }
    var vertex = vertices[param.x];
    var sphere = sphere;
    var date_ = data;
    let sphere_distance = distance(vec4(sphere.x, sphere.y, sphere.z, 1.0), vertex.position);

    // Add gravity
    vertices[param.x].velocity[2] += data.gravity * data.delta_time;

    // Sphere collision
    if sphere_distance <= sphere.radius {
        vertices[param.x].velocity[0] = 0.0;
        vertices[param.x].velocity[1] = 0.0;
        vertices[param.x].velocity[2] = 0.0;
    } else {
        vertices[param.x].velocity[0] += vertices[param.x].resultant[0] * data.delta_time;
        vertices[param.x].velocity[1] += vertices[param.x].resultant[1] * data.delta_time;
        vertices[param.x].velocity[2] += vertices[param.x].resultant[2] * data.delta_time;
    }

    vertices[param.x].position[0] += vertices[param.x].velocity[0] * data.delta_time;
    vertices[param.x].position[1] += vertices[param.x].velocity[1] * data.delta_time;
    vertices[param.x].position[2] += vertices[param.x].velocity[2] * data.delta_time;
}