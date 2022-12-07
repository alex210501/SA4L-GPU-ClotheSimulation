use wgpu_bootstrap::{
    window::Window,
    frame::Frame,
    cgmath,
    application::Application,
    texture::create_texture_bind_group,
    context::Context,
    camera::Camera,
    wgpu,
};

use clothe_simulator::{
    node::Node,
    clothe::Clothe,
};

const GRAVITY: f32 = 9.81;

struct MyApp {
    diffuse_bind_group: wgpu::BindGroup,
    camera_bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    vertices: Vec<Node>,
    indices: Vec<u16>,
}

impl MyApp {
    fn new(context: &Context) -> Self {
        let texture = context.create_srgb_texture("happy-tree.png", include_bytes!("happy-tree.png"));
    
        let diffuse_bind_group = create_texture_bind_group(context, &texture);
    
        let camera = Camera {
            eye: (6.0, 0.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: context.get_aspect_ratio(),
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let (_camera_buffer, camera_bind_group) = camera.create_camera_bind_group(context);
        let clothe = Clothe::new(1.0, 2, &[0.0, 0.0, 0.0]);

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
    
        let vertex_buffer = context.create_buffer(&clothe.vertices, wgpu::BufferUsages::VERTEX);
        let index_buffer = context.create_buffer(&clothe.indices, wgpu::BufferUsages::INDEX);

        Self {
            diffuse_bind_group,
            camera_bind_group,
            pipeline,
            vertex_buffer,
            index_buffer,
            vertices: clothe.vertices.clone(),
            indices: clothe.indices.clone(),
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

    fn update(&mut self, context: &Context, delta_time: f32) {
        // Update the Buffer that contains the delta_time
        self.vertices.iter_mut().for_each(|vertex| {
            vertex.velocity[2] += GRAVITY * delta_time;
            vertex.position[0] += vertex.velocity[0] * delta_time;
            vertex.position[1] += vertex.velocity[1] * delta_time;
            vertex.position[2] += vertex.velocity[2] * delta_time;
        });

        context.update_buffer(&self.vertex_buffer, &self.vertices);
    }
}

fn main() {
    let window = Window::new();

    let context = window.get_context();
    let my_app = MyApp::new(context);

    window.run(my_app);
}
