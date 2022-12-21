use crate::node::Node;
use wgpu_bootstrap::{
    cgmath::{self, prelude::*},
    wgpu,
};

pub struct Clothe {
    length: f32,
    number_square: u32,
    center_x: f32,
    center_y: f32,
    center_z: f32,
    pub vertices: Vec<Node>,
    pub indices: Vec<u16>,
    pub springs: Vec<[u16; 2]>,
    pub rest_distances: Vec<[f32; 3]>,
    pub rest_distances_2: Vec<f32>,
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
            rest_distances: Vec::new(),
            rest_distances_2: Vec::new(),
        };

        instance.construct_vertices();
        instance
    }

    fn add_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push(Node {
            position: [x, y, z],
            normal: [0.0, 0.0, 0.0],
            tangent: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
            velocity: [0.0, 0.0, 1.0],
            resultant: [0.0, 0.0, 0.0],
        });
    }

    fn insert_vertex(&mut self, x: f32, y: f32, z: f32) -> u16 {
        self.add_vertex(x, y, z);
        self.vertices.len() as u16 - 1
    }

    fn add_distance(&mut self, i: u16, j: u16) {
        let vertex_1 = self.vertices.get(i as usize).unwrap();
        let vertex_2 = self.vertices.get(j as usize).unwrap();
        let distances: Vec<f32> = vertex_1
            .position
            .iter()
            .zip(vertex_2.position.iter())
            .map(|(&a, &b)| b - a)
            .collect();

        self.rest_distances
            .push(distances.as_slice().try_into().unwrap());
        self.rest_distances_2.push(vertex_1
            .position
            .iter()
            .zip(vertex_2.position.iter())
            .map(|(&a, &b)| (b - a).powf(2.0))
            .sum::<f32>()
            .sqrt());
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
                let row_offset = x * vertex_length + offset_x;
                let col_offset = y * vertex_length + offset_y;

                let _ = self.insert_vertex(row_offset, -col_offset, self.center_z);
            });
        });

        // Create triangle
        (0..rows).for_each(|row| {
            (0..cols).for_each(|col| {
                let indice = (rows * row + col) as u16;
                let top_left = indice;
                let top_right = indice + 1;
                let bottom_left = indice + rows as u16;
                let bottom_right = indice + rows as u16 + 1;

                // Return on last vertices
                if row == rows - 1 && col == cols - 1 {
                    return;
                }

                // Complete the last row
                if row == rows - 1 {
                        // self.springs.push([indice, indice + 1]); // Right
                        // self.springs.push([indice, indice - rows as u16 + 1]); // Top right
                        // self.add_distance(indice, indice + 1);
                        // self.add_distance(indice, indice - rows as u16 + 1);
                        self.springs.push([indice, indice + 1]); // Right
                        self.springs.push([indice, indice - rows as u16 + 1]); // Bottom Right
                        self.add_distance(indice, indice + 1);
                        self.add_distance(indice, indice - rows as u16 + 1);
                        return;
                }

                // Complete last column
                if col == cols - 1 {
                        self.springs.push([indice, indice + rows as u16]); // Top
                        self.add_distance(indice, indice + rows as u16);
                        return;
                }

                // Put the left neighboor
                // if col > 0 {
                //     self.springs.push([indice, indice - 1]); // Left
                //     self.springs.push([indice, indice + rows as u16 - 1]); // Bottom left
                //     self.add_distance(indice, indice - 1);
                //     self.add_distance(indice, indice + rows as u16 - 1);
                // }

                // Put the top neighboor
                if row > 0 {
                    // self.springs.push([indice, indice - rows as u16]); // Top
                    self.springs.push([indice, indice - rows as u16 + 1]); // Bottom Right
                                                                           // self.add_distance(indice, indice - rows as u16);
                    self.add_distance(indice, indice - rows as u16 + 1);
                }

                // if row > 0 && col > 0 {
                //     self.springs.push([indice, indice - rows as u16 - 1]); // Top left
                //     self.add_distance(indice, indice - rows as u16 - 1);
                // }

                self.springs.push([indice, indice + 1]); // Right
                self.springs.push([indice, indice + rows as u16]); // Top
                self.springs.push([indice, indice + rows as u16 + 1]); // Top Right
                self.add_distance(indice, indice + 1);
                self.add_distance(indice, indice + rows as u16);
                self.add_distance(indice, indice + rows as u16 + 1);

                // Bend springs
                // if col > 1 {
                //     self.springs.push([indice, indice - 2]); // Left
                //     self.add_distance(indice, indice - 2);
                // }

                if col < cols - 2 {
                    self.springs.push([indice, indice + 2]); // Right
                    self.add_distance(indice, indice + 2);
                }

                // if row > 1 {
                //     self.springs.push([indice, indice - 2*rows as u16]); // Top
                //     self.add_distance(indice, indice - 2*rows as u16);
                // }

                if row < rows - 2 {
                    self.springs.push([indice, indice + 2 * rows as u16]); // Top
                    self.add_distance(indice, indice + 2 * rows as u16);
                }

                // Add indices
                self.indices
                    .extend_from_slice(&[top_right, top_left, bottom_left]);
                self.indices
                    .extend_from_slice(&[top_right, bottom_left, bottom_right]);
            });
        });

        // dbg!("vertices: {:?}", &self.vertices);
        // dbg!("indices: {:?}", self.indices);
        // dbg!("springs: {:?}", &self.springs);
        // dbg!("rest_distances: {:?}", &self.rest_distances);
    }
}
