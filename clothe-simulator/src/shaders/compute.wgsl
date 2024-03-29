struct Vertex {
    position: vec3<f32>,
    normal: vec3<f32>,
    velocity: vec3<f32>,
    resultant: vec3<f32>,
    tex_coords: vec3<f32>,
}

struct Sphere {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    friction_factor: f32,
}

struct ClotheData {
    center_x: f32,
    center_y: f32,
    center_z: f32,
    nb_vertices: u32,
    mass: f32,
}

struct Spring {
    links: array<u32, 12>,
    rest_distance: array<f32, 12>,
    current_distance: array<f32, 12>,
}

struct ComputeData {
    spring_contant: f32,
    damping_factor: f32,
    gravity: f32,
    delta_time: f32,
}

@group(0) @binding(0) var<storage, read_write> vertices: array<Vertex>;
@group(0) @binding(1) var<storage, read> springs: array<Spring>;
@group(1) @binding(0) var<uniform> sphere: Sphere;
@group(1) @binding(1) var<uniform> data: ComputeData;
@group(1) @binding(2) var<uniform> clothe_data: ClotheData;


@compute @workgroup_size(255, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    if (param.x > clothe_data.nb_vertices) {
        return;
    }

    var sphere_vec = vec3<f32>(sphere.x, sphere.y, sphere.z);
    var vertex = vertices[param.x];
    var spring = springs[param.x];

    // Reset resultant
    vertices[param.x].resultant = vec3(0.0);

    // Loop on every spring
    for (var i: i32 = 0; i < 12; i++) {
        let vertex_link = vertices[spring.links[i]];

        // If it is the same spring, don't make useless calculations
        if spring.links[i] == param.x {
            continue;
        }

        let norm = (spring.current_distance[i] - spring.rest_distance[i]) * data.spring_contant;
        let spring_force = (vertex_link.position - vertex.position)*norm;
    
        // Calcul resistances
        vertices[param.x].resultant += spring_force - vertices[param.x].velocity * data.damping_factor;
        vertices[param.x].resultant[1] += data.gravity * clothe_data.mass;
    }

    // Add friction with the sphere
    if distance(sphere_vec, vertices[param.x].position) <= sphere.radius {
        let r_n = dot(vertices[param.x].resultant, vertices[param.x].normal) * vertices[param.x].normal;
        let r_t = vertices[param.x].resultant - r_n;

        let one_t = normalize(r_t);

        vertices[param.x].resultant += -min(length(r_t), sphere.friction_factor*length(r_n))*one_t;
    }

    // New velocities and positions
    vertices[param.x].velocity += vertices[param.x].resultant * data.delta_time / clothe_data.mass;
    vertices[param.x].position += vertices[param.x].velocity * data.delta_time;

    // Sphere collision
    if distance(sphere_vec, vertices[param.x].position) < sphere.radius {
        let old_position = vec3(vertices[param.x].position); 

        vertices[param.x].position = sphere_vec + sphere.radius * normalize(vertices[param.x].position - sphere_vec);
        vertices[param.x].velocity = (vertices[param.x].position - old_position) / data.delta_time;
    }
}