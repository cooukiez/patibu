use glam::{Mat4, UVec2, Vec2};
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniform {
    pub proj: [[f32; 4]; 4],
    pub res: [u32; 2],
    pub mouse: [f32; 2],
    pub time: u32,
    pub _padding: [u32; 3],
}

impl Uniform {
    pub fn update_proj(&mut self, res: Vec2) {
        let aspect_ratio = res.x / res.y;
        self.proj = Mat4::orthographic_lh(0.0, aspect_ratio, 0.0, 1.0, 0.0, 100.0).to_cols_array_2d();
    }
}

impl Default for Uniform {
    fn default() -> Self {
        Uniform {
            proj: Mat4::IDENTITY.to_cols_array_2d(),
            res: UVec2::ZERO.to_array(),
            mouse: Vec2::ZERO.to_array(),
            time: 0,
            _padding: [0; 3],
        }
    }
}

