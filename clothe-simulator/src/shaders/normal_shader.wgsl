struct Vertex {
    position: vec3<f32>,
    normal: vec3<f32>,
    velocity: vec3<f32>,
    resultant: vec3<f32>,
    tex_coords: vec4<f32>,
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

@group(0) @binding(0) var<storage, read_write> vertices: array<Vertex>;
@group(0) @binding(1) var<storage, read> springs: array<Spring>;
@group(1) @binding(0) var<uniform> clothe_data: ClotheData;

@compute @workgroup_size(255, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    if (param.x > clothe_data.nb_vertices) {
        return;
    }

    var vertex = vertices[param.x];
    var spring = springs[param.x];

    // Reset normals
    vertices[param.x].normal[0] = 0.0;
    vertices[param.x].normal[1] = 0.0;
    vertices[param.x].normal[2] = 0.0;


    // Calcul normal
    var i: u32 = 1u;
    var j: u32 = 0u;

    // Calcul normal xwith the 8 vertices directly connected
    while i < 8u {
        if spring.links[j] !=  spring.links[i] {
            var vertex_link_1 = vertices[spring.links[j]];
            var vertex_link_2 = vertices[spring.links[i]];

            vertices[param.x].normal += cross(vertex_link_1.position - vertex.position, vertex_link_2.position - vertex.position);
        }

        i++;
        j++;
    }

    vertices[param.x].normal = -normalize(vertices[param.x].normal);
}