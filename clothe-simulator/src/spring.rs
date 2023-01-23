#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Spring {
    pub links: [u32; 12],
    pub rest_distance: [f32; 12],
    pub current_distance: [f32; 12],
}

impl Spring {
    pub fn new() -> Self {
        Self {
            links: Default::default(),
            rest_distance: Default::default(),
            current_distance: Default::default(),
        }
    }
}