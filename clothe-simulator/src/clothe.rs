use crate::node::Node;
pub struct Clothe {
    length: f32,
    number_square: u32,
    center_x: f32,
    center_y: f32,
    center_z: f32,
    pub vertices: Vec<Node>,
    pub indices: Vec<u16>,
    springs: Vec<[u16; 2]>
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
        };

        instance.construct_vertices();
        instance
    }

    fn add_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push(Node { position: [x, y, z], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], velocity: [0.0, 0., 0.5] });
    }

    fn insert_vertex(&mut self, x: f32, y: f32, z: f32) -> u16 {
        self.add_vertex(x, y, z);
        self.vertices.len() as u16 - 1
    }

    fn construct_vertices(&mut self) {
        let vertex_length = self.length / (self.number_square as f32);
        let offset_x = self.center_x - (self.length / 2.0);
        let offset_y = self.center_y - (self.length / 2.0);
        let rows = self.number_square + 1;
        let cols = self.number_square + 1;

        // Create vertices
        (0..rows).map(|x| x as f32).for_each(|y| {
            (0..cols).map(|x| x as f32).for_each(|x| {
                let row_offset = x*vertex_length + offset_x;
                let col_offset = y*vertex_length + offset_y;

                let _ = self.insert_vertex(row_offset, -col_offset, 0.0);
            });
        });

        // Create triangle
        (0..rows - 1).for_each(|row| {
            (0..cols - 1).for_each(|col| {
                let indice = (rows*row + col) as u16;
                let top_left = indice;
                let top_right = indice + 1;
                let bottom_left = indice + rows as u16;
                let bottom_right = indice + rows as u16 + 1;

                // Put the left neighboor
                if col > 0 {
                    self.springs.push([indice, indice - 1]); // Left
                    self.springs.push([indice, indice + rows as u16 - 1]); // Bottom left
                }

                // Put the top neighboor
                if row > 0 {
                    self.springs.push([indice, indice - rows as u16]); // Top
                    self.springs.push([indice, indice - rows as u16 + 1]); // Top Right
                }

                if row > 0 && col > 0 {
                    self.springs.push([indice, indice - rows as u16 - 1]); // Top left
                }

                self.springs.push([indice, indice + 1]); // Right
                self.springs.push([indice, indice + rows as u16]); // Bottom
                self.springs.push([indice, indice + rows as u16 + 1]); // Bottom Right

                // Add indices
                self.indices.extend_from_slice(&[top_right, top_left, bottom_left]);
                self.indices.extend_from_slice(&[top_right, bottom_left, bottom_right]);
            });
        });

        println!("vertices: {:?}", self.vertices);
        println!("indices: {:?}", self.indices);
        println!("springs: {:?}", self.springs);
    }
}
