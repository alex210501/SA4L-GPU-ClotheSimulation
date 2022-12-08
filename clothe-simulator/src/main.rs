use wgpu_bootstrap::{
    application::Application,
    camera::Camera,
    cgmath,
    context::Context,
    default::{Particle, Vertex},
    frame::Frame,
    geometry::icosphere,
    texture::create_texture_bind_group,
    wgpu,
    window::Window,
};

use clothe_simulator::{clothe::Clothe, node::Node};

const SPRING_CONSTANT: f32 = 1.0;
const GRAVITY: f32 = 10.0;
const MASS: f32 = 10.0;

struct Sphere {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
}

struct MyApp {
    diffuse_bind_group: wgpu::BindGroup,
    camera_bind_group: wgpu::BindGroup,
    sphere_pipeline: wgpu::RenderPipeline,
    sphere_buffer: wgpu::Buffer,
    sphere_index_buffer: wgpu::Buffer,
    particle_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    vertices: Vec<Node>,
    indices: Vec<u16>,
    springs: Vec<[u16; 2]>,
    rest_distances: Vec<[f32; 3]>,
    sphere: Sphere,
}

impl MyApp {
    fn new(context: &Context) -> Self {
        let texture =
            context.create_srgb_texture("happy-tree.png", include_bytes!("happy-tree.png"));

        let diffuse_bind_group = create_texture_bind_group(context, &texture);

        let camera = Camera {
            eye: (6.0, 0.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: context.get_aspect_ratio(),
            fovy: 100.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let (_camera_buffer, camera_bind_group) = camera.create_camera_bind_group(context);
        let clothe = Clothe::new(4.0, 4, &[0.0, 0.0, -10.0]);
        let (vertices, indices) = icosphere(1);
        let sphere = Sphere {x: 0.0, y: 0.0, z: 0.0, radius: 1.0};

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

        let sphere_pipeline = context.create_render_pipeline(
            "Render Pipeline",
            include_str!("sphere_shader.wgsl"),
            &[Vertex::desc(), Particle::desc()],
            &[
                &context.texture_bind_group_layout,
                &context.camera_bind_group_layout,
            ],
            wgpu::PrimitiveTopology::TriangleList,
        );

        let particle = Particle {
            position: [sphere.x, sphere.y, sphere.z],
            velocity: [0.0, 0.0, 0.0],
        };

        let vertex_buffer = context.create_buffer(&clothe.vertices, wgpu::BufferUsages::VERTEX);
        let index_buffer = context.create_buffer(&clothe.indices, wgpu::BufferUsages::INDEX);
        let sphere_buffer = context.create_buffer(vertices.as_slice(), wgpu::BufferUsages::VERTEX);
        let sphere_index_buffer =
            context.create_buffer(indices.as_slice(), wgpu::BufferUsages::INDEX);
        let particle_buffer = context.create_buffer(&[particle], wgpu::BufferUsages::VERTEX);
        Self {
            diffuse_bind_group,
            camera_bind_group,
            pipeline,
            sphere_pipeline,
            sphere_buffer,
            sphere_index_buffer,
            particle_buffer,
            vertex_buffer,
            index_buffer,
            vertices: clothe.vertices.clone(),
            indices: clothe.indices.clone(),
            springs: clothe.springs.clone(),
            rest_distances: clothe.rest_distances.clone(),
            sphere,
        }
    }
}

impl Application for MyApp {
    fn render(&self, context: &Context) -> Result<(), wgpu::SurfaceError> {
        let mut frame = Frame::new(context)?;

        {
            let mut render_pass = frame.begin_render_pass(wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(self.indices.len() as u32), 0, 0..1);

            render_pass.set_pipeline(&self.sphere_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.sphere_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.particle_buffer.slice(..));
            render_pass.set_index_buffer(
                self.sphere_index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..(icosphere(1).1.as_slice().len() as u32), 0, 0..1);
        }

        frame.present();

        Ok(())
    }

    fn update(&mut self, context: &Context, delta_time: f32) {
        self.springs.iter().zip(self.rest_distances.iter())
            .for_each(|([i, j], distance)| {
            let resultant: Vec<f32> = {
                let vertex_1 = self.vertices.get(*i as usize).unwrap();
                let vertex_2 = self.vertices.get(*j as usize).unwrap();
                // let distance: f32 = vertex_1.position.iter().zip(vertex_2.position.iter())
                //     .map(|(&a, &b)| (b - a).powf(2.0)).sum::<f32>().sqrt();

                vertex_1.position.iter()
                    .zip(vertex_2.position.iter())
                    .zip(distance.iter())
                    .map(|((&a, &b), &old)| -(b- a + old)*SPRING_CONSTANT).collect()
            };

            let vertex_1 = self.vertices.get_mut(*i as usize).unwrap();

            vertex_1.resultant[0] += resultant.get(0).unwrap();
            vertex_1.resultant[1] += resultant.get(1).unwrap();
            vertex_1.resultant[2] += resultant.get(2).unwrap();
        });

        // Update the Buffer that contains the delta_time
        self.vertices.iter_mut().for_each(|vertex| {
            let sphere_position: [f32; 3] = [self.sphere.x, self.sphere.y, self.sphere.z];
            let distance: f32 = vertex.position.iter().zip(sphere_position.iter())
                .map(|(&a, &b)| (b - a).powf(2.0)).sum::<f32>().sqrt();
            
            if distance <= self.sphere.radius {
                vertex.velocity[0] = 0.0;
                vertex.velocity[1] = 0.0;
                vertex.velocity[2] = 0.0;
            } else {
                vertex.velocity[0] += vertex.resultant[0] * delta_time / MASS;
                vertex.velocity[1] += vertex.resultant[1] * delta_time / MASS;
                vertex.velocity[2] += (vertex.resultant[2] / MASS + GRAVITY) * delta_time;
            }

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
