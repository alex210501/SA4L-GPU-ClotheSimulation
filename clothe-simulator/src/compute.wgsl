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

struct ClotheData {
    center_x: f32,
    center_y: f32,
    center_z: f32,
    nb_vertices: u32,
}

struct Spring {
    links: array<u32, 12u>,
    rest_distance: array<vec4<f32>, 12u>,
    current_distance: f32,
}

struct ComputeData {
    spring_contant: f32,
    damping_factor: f32,
    gravity: f32,
    delta_time: f32,
}

@group(0) @binding(0) var<storage, read_write> vertices: array<Vertex>;
@group(0) @binding(1) var<storage, read_write> springs: array<Spring>;
@group(1) @binding(0) var<uniform> sphere: Sphere;
@group(1) @binding(1) var<uniform> data: ComputeData;
@group(1) @binding(2) var<uniform> clothe_data: ClotheData;


@compute @workgroup_size(255, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    if (param.x > clothe_data.nb_vertices) {
        return;
    }
    var vertex = vertices[param.x];
    var spring = springs[param.x];
    var sphere = sphere;
    var date_ = data;
    let sphere_distance = distance(vec4(sphere.x, sphere.y, sphere.z, 1.0), vertex.position);

    // Reset resultant
    vertices[param.x].resultant[0] = 0.0;
    vertices[param.x].resultant[1] = 0.0;
    vertices[param.x].resultant[2] = 0.0;

    for (var i: i32 = 0; i < 12; i++) {
        let old_distance = distance(vec4(0.0), spring.rest_distance[i]);
        let vertex_1 = vertices[param.x];
        let vertex_2 = vertices[spring.links[i]];

        if spring.links[i] == param.x {
            continue;
        }

        let current_distance =  distance(vertex_1.position, vertex_2.position);
        let norm = (current_distance - old_distance) * data.spring_contant;²
        let spring_force = (vertex_2.position - vertex_1.position)*norm;
    
        vertices[param.x].resultant[0] += spring_force[0]- vertices[param.x].velocity[0] * data.damping_factor;
        vertices[param.x].resultant[1] += spring_force[1]- vertices[param.x].velocity[1] * data.damping_factor;
        vertices[param.x].resultant[2] += spring_force[2]- vertices[param.x].velocity[2] * data.damping_factor;
    }

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