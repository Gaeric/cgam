use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub trait HittableCollection {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

impl HittableCollection for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything: bool = false;
        let mut closest_so_far = ray_t.max;
        let mut temp_rec: HitRecord = Default::default();

        for object in self {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
