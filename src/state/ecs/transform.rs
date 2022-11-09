use glam::{vec3, Mat4, Vec3};

#[derive(Clone, Copy)]
pub struct Transform {
    mat: Mat4,
}

impl Transform {
    pub fn new() -> Self {
        Self::from_translation(vec3(0., 0., 0.))
    }

    pub fn from_translation(translation: Vec3) -> Self {
        let mat = Mat4::from_translation(translation);

        Self { mat }
    }

    pub fn to_scale(self, scale: f32) -> Self {
        Self {
            mat: self.mat * Mat4::from_scale(vec3(scale, scale, scale)),
        }
    }

    pub fn matrix(&self) -> Mat4 {
        self.mat
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}
