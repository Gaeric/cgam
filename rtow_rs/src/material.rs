use std::fmt::Debug;

use crate::{color::Color, hittable::HitRecord, ray::Ray, rtweekend::random_double, vec3::Vec3};

pub trait Material: Debug {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.neal_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

#[derive(Debug)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn reflectance(&self, cosine: f64) -> f64 {
        let r0: f64 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri: f64;
        if rec.front_face {
            ri = 1.0 / self.refraction_index;
        } else {
            ri = self.refraction_index;
        }

        let unit_direction: Vec3 = r_in.direction().unit();
        let cos_theta: f64 = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        println!("ri: {ri}, cos_theta: {cos_theta}, sin_theta: {sin_theta}");

        let cannot_refract: bool = ri * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || self.reflectance(cos_theta) > random_double() {
            direction = unit_direction.reflect(&rec.normal);
        } else {
            direction = unit_direction.refract(&rec.normal, ri);
        }

        *scattered = Ray::new(rec.p, direction);

        return true;
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected: Vec3 = r_in.direction().reflect(&rec.normal);
        reflected = reflected.unit() + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}
