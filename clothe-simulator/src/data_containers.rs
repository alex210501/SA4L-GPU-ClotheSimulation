#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub friction_factor: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ClotheData {
    pub center_x: f32,
    pub center_y: f32,
    pub center_z: f32,
    pub nb_vertices: u32,
    pub mass: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ComputeData {
    pub spring_contant: f32,
    pub damping_factor: f32,
    pub gravity: f32,
    pub delta_time: f32,
}