use bytemuck::{Pod, Zeroable};

use crate::algebra::Vec3;

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct CameraUniforms {
    origin: Vec3,
    _pad0: u32,
    u: Vec3,
    _pad1: u32,
    v: Vec3,
    _pad2: u32,
    w: Vec3,
    _pad3: u32,
}

pub struct Camera {
    uniforms: CameraUniforms,
}

impl Camera {
    pub fn look_at(origin: Vec3, target: Vec3, up: Vec3) -> Camera {
        let w = (origin - target).normalized();
        let u = up.cross(&w).normalized();
        let v = w.cross(&u);

        Camera {
            uniforms: CameraUniforms {
                origin,
                _pad0: 0,
                u,
                _pad1: 1,
                v,
                _pad2: 2,
                w,
                _pad3: 3,
            },
        }
    }

    /// the displacement here refers to movement along the positive w-axis,
    /// where a positive value indicates moving away from the camera,
    /// and a negative value indicates moving toward the camera.
    pub fn zoom(&mut self, displacement: f32) {
        self.uniforms.origin += self.uniforms.w * displacement;
    }

    pub fn pan(&mut self, du: f32, dv: f32) {
        let pan = self.uniforms.u * du + self.uniforms.v * dv;
        self.uniforms.origin += pan;
    }

    pub fn uniforms(&self) -> &CameraUniforms {
        &self.uniforms
    }
}
