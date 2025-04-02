use std::f32::consts::{FRAC_PI_2, PI};

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
    center: Vec3,
    up: Vec3,
    distance: f32,
    azimuth: f32,
    altitude: f32,
}

impl Camera {
    pub fn with_spherical_coords(
        center: Vec3,
        up: Vec3,
        distance: f32,
        azimuth: f32,
        altitude: f32,
    ) -> Camera {
        let mut camera = Camera {
            uniforms: CameraUniforms::zeroed(),
            center,
            up,
            distance,
            azimuth,
            altitude,
        };
        camera.calculate_uniforms();
        camera
    }

    /// the displacement here refers to movement along the positive w-axis,
    /// where a positive value indicates moving away from the camera,
    /// and a negative value indicates moving toward the camera.
    pub fn zoom(&mut self, displacement: f32) {
        // prevent negative distance
        self.distance = (self.distance - displacement).max(0.0);
        self.uniforms.origin = self.center + self.uniforms.w * self.distance;
    }

    pub fn pan(&mut self, du: f32, dv: f32) {
        let pan = self.uniforms.u * du + self.uniforms.v * dv;
        self.uniforms.origin += pan;
    }

    pub fn uniforms(&self) -> &CameraUniforms {
        &self.uniforms
    }

    pub fn orbit(&mut self, du: f32, dv: f32) {
        const MAX_ALT: f32 = FRAC_PI_2 - 1e-6;
        self.altitude = (self.altitude + dv).clamp(-MAX_ALT, MAX_ALT);
        self.azimuth += du;
        self.azimuth %= 2.0 * PI;
        self.calculate_uniforms();
    }

    fn calculate_uniforms(&mut self) {
        // let w = Vec3::new(0.0, 0.0, 1.0);
        let origin = Vec3::new(
            self.altitude.cos() * self.azimuth.sin(),
            self.altitude.sin(),
            self.altitude.cos() * self.azimuth.cos(),
        );
        let w = origin;
        let origin = self.center + w * self.distance;
        let u = self.up.cross(&w).normalized();
        let v = w.cross(&u);
        self.uniforms.origin = origin;
        self.uniforms.u = u;
        self.uniforms.v = v;
        self.uniforms.w = w;
    }
}
