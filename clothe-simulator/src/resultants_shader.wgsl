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
@group(0) @binding(1) var<storage, read> springs: array<vec2<u32>>;
@group(0) @binding(2) var<storage, read> rest_distances: array<f32>;
@group(1) @binding(0) var<uniform> data: ComputeData;

@compute @workgroup_size(1, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    if (param.x > u32(384)) {
        return;
    }

    if (param.x == u32(0)) {
        for (var i = 0u; i < 81u; i++) {
            vertices[i].resultant[0] = 0.0;
            vertices[i].resultant[1] = 0.0;
            vertices[i].resultant[2] = 0.0;
        }
    }

    for (var i = 0u; i < 384u; i++) {
        let old_distance = rest_distances[i];
        let spring = springs[i];
        let vertex_1 = vertices[spring[0]];
        let vertex_2 = vertices[spring[1]];

        let current_distance = sqrt(pow(vertex_1.position[0] - vertex_2.position[0], 2.0) + 
        pow(vertex_1.position[1] - vertex_2.position[1], 2.0) +
        pow(vertex_1.position[2] - vertex_2.position[2], 2.0));
        let norm = (current_distance - old_distance) * data.spring_contant;
        let vec_from_1_to_2 = vec3(vertex_2.position[0] -vertex_1.position[0], 
            vertex_2.position[1] - vertex_1.position[1], 
            vertex_2.position[2] - vertex_1.position[2]);
        let spring_force = vec_from_1_to_2*norm;
    
        vertices[spring[0]].resultant[0] += spring_force[0]- vertices[spring[0]].velocity[0] * data.damping_factor;
        vertices[spring[0]].resultant[1] += spring_force[1]- vertices[spring[0]].velocity[1] * data.damping_factor;
        vertices[spring[0]].resultant[2] += spring_force[2]- vertices[spring[0]].velocity[2] * data.damping_factor;

        vertices[spring[1]].resultant[0] -= spring_force[0] + vertices[spring[1]].velocity[0] * data.damping_factor;
        vertices[spring[1]].resultant[1] -= spring_force[1] + vertices[spring[1]].velocity[1] * data.damping_factor;
        vertices[spring[1]].resultant[2] -= spring_force[2] + vertices[spring[1]].velocity[2] * data.damping_factor;

        vertices[spring[0]].velocity[2] = 1.0;
        vertices[spring[1]].velocity[2] = 1.0;
    }
    // var spring = springs[param.x];
    // let old_distance = rest_distances[param.x];
    // let vertex_1 = vertices[spring[0]];
    // let vertex_2 = vertices[spring[1]];
    // var data_ = data;
    
    // let current_distance = sqrt(pow(vertex_1.position[0] - vertex_2.position[0], 2.0) + 
    //     pow(vertex_1.position[1] - vertex_2.position[1], 2.0) +
    //     pow(vertex_1.position[2] - vertex_2.position[2], 2.0));
    // let norm = (current_distance - old_distance*1.0); //  * data.spring_contant;
    // let vec_from_1_to_2 = vec3(vertex_2.position[0] -vertex_1.position[0], 
    //     vertex_2.position[1] - vertex_1.position[1], 
    //     vertex_2.position[2] - vertex_1.position[2]);
    // let spring_force = vec_from_1_to_2*norm;
    
    // vertices[spring[0]].resultant[0] += spring_force[0] - vertex_1.velocity[0] * data.damping_factor;
    // vertices[spring[0]].resultant[1] += spring_force[1] - vertex_1.velocity[1] * data.damping_factor;
    // vertices[spring[0]].resultant[2] += spring_force[2] - vertex_1.velocity[2] * data.damping_factor;

    // vertices[spring[1]].resultant[0] -= spring_force[0] + vertex_2.velocity[0] * data.damping_factor;
    // vertices[spring[1]].resultant[1] -= spring_force[1] + vertex_2.velocity[1] * data.damping_factor;
    // vertices[spring[1]].resultant[2] -= spring_force[2] + vertex_2.velocity[2] * data.damping_factor;

    // vertices[spring[0]].velocity[0] = 0.0;
    // vertices[spring[0]].velocity[1] = 0.0;
    // vertices[spring[0]].velocity[2] = 1.0;
    // vertices[spring[1]].velocity[2] = 1.0;
}