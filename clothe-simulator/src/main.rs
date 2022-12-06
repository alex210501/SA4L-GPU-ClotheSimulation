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

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, -0.0, 0.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [1.0, -0.0, 0.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0., 0.0], },
    Vertex { position: [0.0, -1.0, 0.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [1.0, -1.0, 0.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    // Vertex { position: [0.35966998, -0.3473291, 0.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.85967, 0.84732914], },
    // Vertex { position: [0.44147372, 0.2347359, 0.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.9414737, 0.2652641], },
];

const INDICES: &[u16] = &[1, 0, 2, 1, 2, 3];

struct FabricConstructor {
    length: f32,
    number_square: u32,
    center_x: f32,
    center_y: f32,
    center_z: f32,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    indices_map: HashMap<String, u16>,
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
        };

        instance.construct_vertices();
        instance
    }

    fn add_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push(Vertex { position: [x, y, z], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0] });
    }

    fn get_indice_vertex_position(&mut self, x: f32, y: f32, z: f32) -> u16 {
        let vertice_key = format!("{}-{}", x, y);

        match self.indices_map.get(&vertice_key) {
            Some(&indice) => indice,
            None => {
                let vertices_len = self.vertices.len() as u16;

                self.indices_map.insert(vertice_key,  vertices_len);
                self.add_vertex(x, y, z);
                vertices_len
            }
        }
    }

    fn construct_vertices(&mut self) {
        let vertex_length = self.length / (self.number_square as f32);
        let offset_x = self.center_x - (self.length / 2.0);
        let offset_y = self.center_y - (self.length / 2.0);

        (0..self.number_square).map(|x| x as f32).for_each(|row| {
            (0..self.number_square).map(|x| x as f32).for_each(|col| {
                let row_offset = row*vertex_length + offset_x;
                let col_offset = col*vertex_length + offset_y;
                let top_left = self.get_indice_vertex_position(row_offset, -col_offset, 0.0);
                let top_right = self.get_indice_vertex_position(row_offset + vertex_length, -col_offset, 0.0);
                let bottom_left = self.get_indice_vertex_position(row_offset, -(col_offset + vertex_length), 0.0);
                let bottom_right = self.get_indice_vertex_position(row_offset + vertex_length, -(col_offset + vertex_length), 0.0);

                self.indices.extend_from_slice(&[top_right, top_left, bottom_left]);
                self.indices.extend_from_slice(&[top_right, bottom_left, bottom_right]);
            });
        });

        println!("vertices: {:?}", self.vertices);
        println!("indices: {:?}", self.indices);
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
        let mut fabric_constructor = FabricConstructor::new(1.0, 10, &[0.0, 0.0, 0.0]);

        let pipeline = context.create_render_pipeline(
            "Render Pipeline",
            include_str!("shader.wgsl"),
            &[Vertex::desc()],
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
