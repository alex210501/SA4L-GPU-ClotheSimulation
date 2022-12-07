use std::collections::HashMap;

use wgpu_bootstrap::{
    window::Window,
    frame::Frame,
    cgmath,
    application::Application,
    texture::create_texture_bind_group,
    context::Context,
    camera::Camera,
    default::Vertex,
    wgpu,
};

use clothe_simulator::node::Node;

struct FabricConstructor {
    length: f32,
    number_square: u32,
    center_x: f32,
    center_y: f32,
    center_z: f32,
    vertices: Vec<Node>,
    indices: Vec<u16>,
    indices_map: HashMap<String, u16>,
    springs: Vec<[u16; 2]>
}

impl FabricConstructor {
    fn new(length: f32, number_square: u32, center: &[f32; 3]) -> Self {
        let mut instance = Self { 
            length, 
            number_square, 
            vertices: Vec::new(), 
            indices: Vec::new(), 
            indices_map: HashMap::new(), 
            center_x: center[0],
            center_y: center[1],
            center_z: center[2],
            springs: Vec::new(),
        };

        instance.construct_vertices();
        instance
    }

    fn add_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push(Node { position: [x, y, z], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], velocity: [0.0, 0.0, 0.0] });
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

struct MyApp {
    diffuse_bind_group: wgpu::BindGroup,
    camera_bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices: Vec<u16>
}

impl MyApp {
    fn new(context: &Context) -> Self {
        let texture = context.create_srgb_texture("happy-tree.png", include_bytes!("happy-tree.png"));
    
        let diffuse_bind_group = create_texture_bind_group(context, &texture);
    
        let camera = Camera {
            eye: (0.0, 0.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: context.get_aspect_ratio(),
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let (_camera_buffer, camera_bind_group) = camera.create_camera_bind_group(context);
        let mut fabric_constructor = FabricConstructor::new(1.0, 2, &[0.0, 0.0, 0.0]);

        let pipeline = context.create_render_pipeline(
            "Render Pipeline",
            include_str!("shader.wgsl"),
            &[Node::desc()],
            &[
                &context.texture_bind_group_layout,
                &context.camera_bind_group_layout,
            ],
            wgpu::PrimitiveTopology::TriangleList
        );
    
        let vertex_buffer = context.create_buffer(&fabric_constructor.vertices, wgpu::BufferUsages::VERTEX);
        let index_buffer = context.create_buffer(&fabric_constructor.indices, wgpu::BufferUsages::INDEX);

        Self {
            diffuse_bind_group,
            camera_bind_group,
            pipeline,
            vertex_buffer,
            index_buffer,
            indices: fabric_constructor.indices.clone(),
        }
    }
}

impl Application for MyApp {
    fn render(&self, context: &Context) -> Result<(), wgpu::SurfaceError> {
        let mut frame = Frame::new(context)?;

        {
            let mut render_pass = frame.begin_render_pass(wgpu::Color {r: 0.1, g: 0.2, b: 0.3, a: 1.0});
            
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(self.indices.len() as u32), 0, 0..1);
        }

        frame.present();

        Ok(())
    }
}

fn main() {
    let window = Window::new();

    let context = window.get_context();
    let my_app = MyApp::new(context);

    window.run(my_app);
}
