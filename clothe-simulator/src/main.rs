use wgpu_bootstrap::{
    application::Application,
    camera::Camera,
    cgmath,
    computation::Computation,
    context::Context,
    default::{Particle, Vertex},
    frame::Frame,
    geometry::icosphere,
    texture::create_texture_bind_group,
    wgpu,
    window::Window,
};

use clothe_simulator::{
    clothe::Clothe, 
    node::Node, 
    data_containers:: {Sphere, ClotheData, ComputeData}
};

// Sphere parameters
const SPHERE: Sphere = Sphere {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    radius: 1.05,
    friction_factor: 0.8,
};

// Parameters
const WORKER_SIZE: u32 = 255;
const SPHERE_ORDER: u32 = 3;
const SPRING_CONSTANT: f32 = 1000.0;
const GRAVITY: f32 = -9.81;
const MASS: f32 = 0.5;
const CLOTH_SIZE: f32 = 2.5;
const NUMBER_SQUARES: u32 = 21;
const DAMPING_FACTOR: f32 = 0.5;
const CLOTHE_CENTER: &[f32; 3] = &[0.0, 2.0, 0.0]; // [x, y, z]

// Function to calcul the number of workers to launch
fn get_workers(nb: u32) -> u32 {
    if nb % WORKER_SIZE == 0 {
        nb / WORKER_SIZE
    } else {
        (nb / WORKER_SIZE) + 1
    }
}

struct MyApp {
    sphere_bind_group: wgpu::BindGroup,
    clothe_diffuse_bind_group: wgpu::BindGroup,
    camera_bind_group: wgpu::BindGroup,
    sphere_pipeline: wgpu::RenderPipeline,
    sphere_buffer: wgpu::Buffer,
    sphere_index_buffer: wgpu::Buffer,
    particle_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    compute_pipeline: wgpu::ComputePipeline,
    distance_pipeline: wgpu::ComputePipeline,
    normal_pipeline: wgpu::ComputePipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    compute_vertex_bind_group: wgpu::BindGroup,
    compute_data_buffer: wgpu::Buffer,
    compute_data_bind_group: wgpu::BindGroup,
    compute_distance_bind_group: wgpu::BindGroup,
    distance_vertex_bind_group: wgpu::BindGroup,
    normal_vertex_bind_group: wgpu::BindGroup,
    normal_bind_group: wgpu::BindGroup,
    indices: Vec<u16>,
    clothe_data: ClotheData,
}

impl MyApp {
    fn new(context: &Context) -> Self {
        // Add texture for the sphere and the clothe
        let sphere_texture = context.create_srgb_texture("golf-ball.jpg", 
            include_bytes!("textures/golf-ball.jpg"));
        let clothe_texture = context.create_srgb_texture("clothe-texture.jpg", 
            include_bytes!("textures/clothe-texture.jpg"));

        // Create bind groups for texture
        let sphere_bind_group = create_texture_bind_group(context, &sphere_texture);
        let clothe_diffuse_bind_group = create_texture_bind_group(context, &clothe_texture);

        // Configure the camera parameters
        let camera = Camera {
            eye: (3.0, 1.0, 0.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: context.get_aspect_ratio(),
            fovy: 100.0,
            znear: 0.1,
            zfar: 100.0,
        };
        let (_camera_buffer, camera_bind_group) = camera.create_camera_bind_group(context);

        /**********************************************************************************
         *                                Sphere Render
         **********************************************************************************/
         let particle = Particle {
            position: [SPHERE.x, SPHERE.y, SPHERE.z],
            velocity: [0.0, 0.0, 0.0],
        };
        let (vertices, indices) = icosphere(SPHERE_ORDER);
        let sphere_pipeline = context.create_render_pipeline(
            "Sphere Render Pipeline",
            include_str!("shaders/sphere_shader.wgsl"),
            &[Vertex::desc(), Particle::desc()],
            &[
                &context.texture_bind_group_layout,
                &context.camera_bind_group_layout,
            ],
            wgpu::PrimitiveTopology::TriangleList,
        );

        // Create sphere buffer
        let sphere_buffer = context.create_buffer(vertices.as_slice(), wgpu::BufferUsages::VERTEX);
        let sphere_index_buffer =
            context.create_buffer(indices.as_slice(), wgpu::BufferUsages::INDEX);
        let particle_buffer = context.create_buffer(&[particle], wgpu::BufferUsages::VERTEX);
        
        /**********************************************************************************
         *                               Clothe Render
         **********************************************************************************/
        let clothe = Clothe::new(CLOTH_SIZE, NUMBER_SQUARES, CLOTHE_CENTER);
        let clothe_data: ClotheData = ClotheData {
            center_x: clothe.center_x,
            center_y: clothe.center_y,
            center_z: clothe.center_z,
            nb_vertices: clothe.nb_vertices,
            mass: MASS,
        };

        // Create pipeline to render the clothe
        let pipeline = context.create_render_pipeline(
            "Clothe Render Pipeline",
            include_str!("shaders/shader.wgsl"),
            &[Node::desc()],
            &[
                &context.texture_bind_group_layout,
                &context.camera_bind_group_layout,
            ],
            wgpu::PrimitiveTopology::TriangleList,
        );

        // Create buffer that contains the vertices and springs
        let vertex_buffer = context.create_buffer(
            &clothe.vertices,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
        );
        let spring_buffer = context.create_buffer(
            &clothe.springs,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
        );
        let index_buffer = context.create_buffer(&clothe.indices, wgpu::BufferUsages::INDEX);

        /**********************************************************************************
         *                               Compute Shaders
         **********************************************************************************/
        let compute_data = ComputeData {
            spring_contant: SPRING_CONSTANT,
            damping_factor: DAMPING_FACTOR,
            gravity: GRAVITY,
            delta_time: 0.0,
        };

        // Create the pipelines
        let compute_pipeline = context.create_compute_pipeline("Compute Pipeline", 
            include_str!("shaders/compute.wgsl"));
        let distance_pipeline = context.create_compute_pipeline("Distance Pipeline",
            include_str!("shaders/distance_shader.wgsl"));
        let normal_pipeline = context.create_compute_pipeline("Normal Pipeline",
            include_str!("shaders/normal_shader.wgsl"));

        // Create the buffers
        let compute_sphere_buffer = context.create_buffer(&[SPHERE], wgpu::BufferUsages::UNIFORM);
        let compute_data_buffer = context.create_buffer(&[compute_data], 
            wgpu::BufferUsages::UNIFORM);
        let compute_clothe_data_buffer = context.create_buffer(&[clothe_data], 
            wgpu::BufferUsages::UNIFORM);

        // Create the bind groups
        let compute_vertex_bind_group = context.create_bind_group(
            "Compute Bind Group",
            &compute_pipeline.get_bind_group_layout(0),
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: spring_buffer.as_entire_binding(),
                },
            ],
        );

        let distance_vertex_bind_group = context.create_bind_group(
            "Distance Bind Group",
            &distance_pipeline.get_bind_group_layout(0),
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: spring_buffer.as_entire_binding(),
                },
            ],
        );

        let normal_vertex_bind_group = context.create_bind_group(
            "Normal Bind Group",
            &normal_pipeline.get_bind_group_layout(0),
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: spring_buffer.as_entire_binding(),
                },
            ],
        );

        
        let compute_data_bind_group = context.create_bind_group(
            "Compute Data",
            &compute_pipeline.get_bind_group_layout(1),
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: compute_sphere_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: compute_data_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: compute_clothe_data_buffer.as_entire_binding(),
                },
            ],
        );

        let compute_distance_bind_group = context.create_bind_group(
            "Compute Data",
            &distance_pipeline.get_bind_group_layout(1),
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: compute_clothe_data_buffer.as_entire_binding(),
                },
            ],
        );

        let normal_bind_group = context.create_bind_group(
            "Normal Data",
            &normal_pipeline.get_bind_group_layout(1),
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: compute_clothe_data_buffer.as_entire_binding(),
                },
            ],
        );

        Self {
            sphere_bind_group,
            clothe_diffuse_bind_group,
            camera_bind_group,
            pipeline,
            sphere_pipeline,
            sphere_buffer,
            sphere_index_buffer,
            particle_buffer,
            compute_pipeline,
            distance_pipeline,
            normal_pipeline,
            vertex_buffer,
            index_buffer,
            compute_vertex_bind_group,
            compute_data_bind_group,
            compute_data_buffer,
            compute_distance_bind_group,
            distance_vertex_bind_group,
            normal_vertex_bind_group,
            normal_bind_group,
            indices: clothe.indices.clone(),
            clothe_data,
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

            // Clothe render pipeline
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.clothe_diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(self.indices.len() as u32), 0, 0..1);

            // Sphere render pipeline
            render_pass.set_pipeline(&self.sphere_pipeline);
            render_pass.set_bind_group(0, &self.sphere_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.sphere_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.particle_buffer.slice(..));
            render_pass.set_index_buffer(self.sphere_index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,);
            render_pass.draw_indexed(0..(icosphere(SPHERE_ORDER).1.as_slice().len() as u32), 0, 0..1);
        }

        frame.present();

        Ok(())
    }

    fn update(&mut self, context: &Context, delta_time: f32) {
        let compute_data = ComputeData {
            spring_contant: SPRING_CONSTANT,
            damping_factor: DAMPING_FACTOR,
            gravity: GRAVITY * 0.2,
            delta_time: delta_time,
        };
        context.update_buffer(&self.compute_data_buffer, &[compute_data]);

        let compute_nb: u32 = get_workers(self.clothe_data.nb_vertices);
        let mut computation = Computation::new(context);

        {
            let mut compute_pass = computation.begin_compute_pass();

            // Normal pipeline
            compute_pass.set_pipeline(&self.normal_pipeline);
            compute_pass.set_bind_group(0, &self.normal_vertex_bind_group, &[]);
            compute_pass.set_bind_group(1, &self.normal_bind_group, &[]);
            compute_pass.dispatch_workgroups(compute_nb, 1, 1);

            // Distance pipeline
            compute_pass.set_pipeline(&self.distance_pipeline);
            compute_pass.set_bind_group(0, &self.distance_vertex_bind_group, &[]);
            compute_pass.set_bind_group(1, &self.compute_distance_bind_group, &[]);
            compute_pass.dispatch_workgroups(compute_nb, 1, 1);

            // Compute pipeline
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.compute_vertex_bind_group, &[]);
            compute_pass.set_bind_group(1, &self.compute_data_bind_group, &[]);
            compute_pass.dispatch_workgroups(compute_nb, 1, 1);
        }

        computation.submit();
    }
}

fn main() {
    let window = Window::new();

    let context = window.get_context();
    let my_app = MyApp::new(context);

    window.run(my_app);
}
