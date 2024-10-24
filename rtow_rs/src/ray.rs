use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    fn origin(&self) -> &Point3 {
        &self.orig
    }

    fn direction(&self) -> &Vec3 {
        &self.dir
    }

    fn at(&self, t: f64) -> Point3 {
        &self.orig + &(&self.dir * t)
    }
}
