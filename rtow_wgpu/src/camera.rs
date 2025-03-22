use bytemuck::{Pod, Zeroable};

use crate::algebra::Vec3;

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct CameraUniforms {
    origin: Vec3,
}

pub struct Camera {
    uniforms: CameraUniforms,
}

impl Camera {
    pub fn new(origin: Vec3) -> Camera {
        Camera {
            uniforms: CameraUniforms { origin },
        }
    }

    pub fn uniforms(&self) -> &CameraUniforms {
        &self.uniforms
    }
}
