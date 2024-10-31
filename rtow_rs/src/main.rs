use camera::Camera;
use hittable::Hittable;

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod vec3;
mod sphere;

fn main() {
    const DELAY_TIME: u64 = 0;

    let mut camera = Camera::new();

    camera.delay = DELAY_TIME;
}
