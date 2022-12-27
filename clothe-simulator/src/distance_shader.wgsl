struct Vertex {
    position: vec3<f32>,
    normal: vec3<f32>,
    velocity: vec3<f32>,
    resultant: vec3<f32>
}

struct Spring {
    links: array<u32, 12>,
    rest_distance: array<f32, 12>,
    current_distance: array<f32, 12>,
}

struct ClotheData {
    center_x: f32,
    center_y: f32,
    center_z: f32,
    nb_vertices: u32,
    mass: f32,
}

@group(0) @binding(0) var<storage, read> vertices: array<Vertex>;
@group(0) @binding(1) var<storage, read_write> springs: array<Spring>;
@group(1) @binding(0) var<uniform> clothe_data: ClotheData;

@compute @workgroup_size(255, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    if (param.x > clothe_data.nb_vertices) {
        return;
    }

    var vertex = vertices[param.x];
    var spring = springs[param.x];

    for (var i: i32 = 0; i < 12; i++) {
        let vertex_link = vertices[spring.links[i]];

        if spring.links[i] == param.x {
            continue;
        }

        springs[param.x].current_distance[i] =  distance(vertex.position, vertex_link.position);
    }
}