use crate::{
    node::Node,
    spring::Spring,
};

pub struct Clothe {
    length: f32,
    number_square: u32,
    pub center_x: f32,
    pub center_y: f32,
    pub center_z: f32,
    pub nb_vertices: u32,
    pub vertices: Vec<Node>,
    pub indices: Vec<u16>,
    pub springs: Vec<Spring>,
}

impl Clothe {
    pub fn new(length: f32, number_square: u32, center: &[f32; 3]) -> Self {
        let mut instance = Self {
            length,
            number_square,
            vertices: Vec::new(),
            indices: Vec::new(),
            center_x: center[0],
            center_y: center[1],
            center_z: center[2],
            springs: Vec::new(),
            nb_vertices: 0,
        };

        instance.construct_vertices();
        instance
    }

    fn insert_vertex(&mut self, x: f32, y: f32, z: f32, x_tex_coords: f32, y_tex_coords: f32) -> u16 {
        self.vertices.push(Node {
            position: [x, y, z, 1.0],
            normal: [0.0, 0.0, 0.0, 1.0],
            velocity: [0.0, 0.0, 0.0, 1.0],
            resultant: [0.0, 0.0, 0.0, 1.0],
            tex_coords: [x_tex_coords, y_tex_coords, 1.0, 1.0],
        });
        self.vertices.len() as u16 - 1
    }

    fn get_norm_distance(&self, i: u32, j: u32) -> f32 {
        let vertex_1 = self.vertices.get(i as usize).unwrap();
        let vertex_2 = self.vertices.get(j as usize).unwrap();

         vertex_1
            .position
            .iter()
            .zip(vertex_2.position.iter())
            .map(|(&a, &b)| (b - a).powf(2.0))
            .sum::<f32>()
            .sqrt()
    }

    fn construct_vertices(&mut self) {
        let vertex_length = self.length / (self.number_square as f32);
        let offset_x = self.center_x - (self.length / 2.0);
        let offset_z = self.center_z - (self.length / 2.0);
        let rows = self.number_square + 1;
        let cols = self.number_square + 1;

        // Create vertices
        (0..rows).map(|x| x as f32).for_each(|z| {
            (0..cols).map(|x| x as f32).for_each(|x| {
                let row_offset = x * vertex_length + offset_x;
                let col_offset = z * vertex_length + offset_z;
                let x_coords = x / (rows as f32 - 1.0);
                let y_coords = z / (cols as f32 - 1.0);

                let _ = self.insert_vertex(row_offset, self.center_y, col_offset, 
                    x_coords, y_coords);
            });
        });

        // Create indices
        (0..rows - 1).for_each(|row| {
            (0..cols - 1).for_each(|col| {
                let indice = (rows * row + col) as u32;
                let top_left = indice as u16;
                let top_right = (indice as u16) + 1;
                let bottom_left = (indice + rows) as u16;
                let bottom_right = ((indice + rows) as u16) + 1;

                self.indices.extend_from_slice(&[top_right, top_left, bottom_left]);
                self.indices.extend_from_slice(&[top_right, bottom_left, bottom_right]);
                self.indices.extend_from_slice(&[top_left, top_right, bottom_left]);
                self.indices.extend_from_slice(&[bottom_left, top_right, bottom_right]);
            });
        });

        // Set the size of the springs vector
        self.nb_vertices = self.vertices.len() as u32;
        self.springs = vec![Spring::new(); self.nb_vertices as usize];
        // Create triangle
        (0..rows).for_each(|row| {
            (0..cols).for_each(|col| {
                let indice = (rows * row + col) as u32;
                let mut spring = Spring::new();

                for i in 0..12 {
                    spring.links[i] = indice;
                }

                // If it is not the first row, we add the top parts
                if row > 0 {
                    spring.links[1] = indice - cols; // Top
                    spring.rest_distance[1] = self.get_norm_distance(indice, indice - cols); // Top
                    if col > 0 {
                        spring.links[0] = indice - 1 - cols; // Top left
                        spring.rest_distance[0] = self.get_norm_distance(indice, indice - 1 - cols); // Top left
                    }
                    if col < cols - 1 {
                        spring.links[2] = indice + 1 - cols; // Top right
                        spring.rest_distance[2] = self.get_norm_distance(indice, indice + 1 - cols); // Top right
                    }
                }

                // If it is not the last row, we can add the bottom part
                if row < rows - 1 {
                    spring.links[5] = indice + cols; // Bottom
                    spring.rest_distance[5] = self.get_norm_distance(indice, indice + cols); // Bottom

                    if col > 0 {
                        spring.links[6] = indice - 1 + cols; // Bottom left
                        spring.rest_distance[6] = self.get_norm_distance(indice, indice - 1 + cols); // Bottom left
                    }
                    if col < cols - 1 {
                        spring.links[4] = indice + 1 + cols; // Bottom right
                        spring.rest_distance[4] = self.get_norm_distance(indice, indice + 1 + cols); // Bottom right
                    }
                }

                // If it is not the first column, we add `left`
                if col > 0 {
                    spring.links[7] = indice - 1; // Left
                    spring.rest_distance[7] = self.get_norm_distance(indice, indice - 1);
                    // Left
                }

                // If it is not the last column, we add `right`
                if col < cols - 1 {
                    spring.links[3] = indice + 1; // Right
                    spring.rest_distance[3] = self.get_norm_distance(indice, indice + 1);
                    // Right
                }

                // Add blend springs
                if col > 1 {
                    spring.links[8] = indice - 2; // Blend left
                    spring.rest_distance[8] = self.get_norm_distance(indice, indice - 2); // Blend left
                }
                if col < cols - 2 {
                    spring.links[9] = indice + 2; // Blend right
                    spring.rest_distance[9] = self.get_norm_distance(indice, indice + 2); // Blend right
                }
                if row > 1 {
                    spring.links[10] = indice - 2 * cols; // Blend top
                    spring.rest_distance[10] = self.get_norm_distance(indice, indice - 2 * cols); // Blend top
                }
                if row < rows - 2 {
                    spring.links[11] = indice + 2 * cols; // Blend bottom
                    spring.rest_distance[11] = self.get_norm_distance(indice, indice + 2 * cols); // Blend bottom
                }

                self.springs[indice as usize] = spring;
            });
        });

        // dbg!("vertices: {:?}", &self.vertices);
        // dbg!("indices: {:?}", &self.indices);
        // dbg!("springs: {:?}", &self.springs);
        // dbg!("rest_distances: {:?}", &self.rest_distances);
    }
}
