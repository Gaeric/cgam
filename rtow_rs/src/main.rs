use camera::Camera;
use color::Color;
use hittable::Hittable;
use material::Lambertian;
use sphere::Sphere;
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

fn main() {
    const DELAY_TIME: u64 = 0;

    let mut camera = Camera::new();

    let center: Point3 = Point3::random_random();
    let albedo: Color = Color::random_random() * Color::random_random();
    let lambertian_material = Lambertian { albedo };
    let sphere = Sphere::new(center, 2.0, &lambertian_material);

    camera.delay = DELAY_TIME;

    camera.render(sphere);
}
