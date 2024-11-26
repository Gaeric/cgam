use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug)]
pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    mat: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, mat: &'a dyn Material) -> Self {
        Self {
            center: center.clone(),
            radius,
            mat,
        }
    }
}

impl<'b> Hittable for Sphere<'b> {
    fn hit<'a>(&'a self, r: &Ray, ray_t: Interval, rec: &mut HitRecord<'a>) -> bool {
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
        rec.mat = Some(self.mat);
        true
    }
}
