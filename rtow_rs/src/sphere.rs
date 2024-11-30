use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center: center.clone(),
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            let root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::Lambertian;
    use crate::Vec3;

    #[test]
    fn test_sphere_hit() {
        let center = Vec3::new(0.0, 0.0, -5.0);
        let radius = 1.0;
        let ground_material = Rc::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        });
        let sphere = Sphere {
            center,
            radius,
            mat: ground_material,
        };
        let ray_origin = Vec3::new(0.0, 0.0, 0.0);
        let ray_direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(ray_origin, ray_direction);
        let ray_t = Interval::new(0.0, 100.0);

        let mut hit_record = HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            mat: None,
        };

        let hit = sphere.hit(&ray, ray_t, &mut hit_record);

        assert!(hit, "The ray should hit the sphere.");
        assert!(hit_record.t > 0.0, "Intersection t should be greater than 0.");
        assert_eq!(hit_record.p, Vec3::new(0.0, 0.0, -4.0), "Intersection point should be at (0, 0, -4).");
        assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, 1.0), "Normal should be (0, 0, 1).");
        assert!(hit_record.mat.is_some(), "Material should be set.");
    }

    #[test]
    fn test_sphere_hitrecord() {
        let center = Vec3::new(0.0, 0.0, -5.0);
        let radius = 1.0;
        let ground_material = Rc::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        });
        let sphere = Sphere {
            center,
            radius,
            mat: ground_material,
        };
        let ray_origin = Vec3::new(0.0, 0.0, -5.5);
        let ray_direction = Vec3::new(0.0, 0.0, 1.0);
        let ray = Ray::new(ray_origin, ray_direction);
        let ray_t = Interval::new(0.0, 100.0);

        let mut hit_record = HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            mat: None,
        };

        let hit = sphere.hit(&ray, ray_t, &mut hit_record);

        assert!(hit, "The ray should hit the sphere.");
        assert!(hit_record.t > 0.0, "Intersection t should be greater than 0.");
        assert_eq!(hit_record.p, Vec3::new(0.0, 0.0, -4.0), "Intersection point should be at (0, 0, -4).");
        assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, -1.0), "Normal should be (0, 0, -1.0).");
        assert!(hit_record.mat.is_some(), "Material should be set.");
    }
}
