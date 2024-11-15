use camera::Camera;
use color::Color;
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
mod rtweekend;

fn main() {
    let mut camera = Camera::new();

    let center: Point3 = Point3::new(0.0, 0.0, -1.0);
    let albedo: Color = Color::random_random() * Color::random_random();
    let lambertian_material = Lambertian { albedo };
    let sphere = Sphere::new(center, 0.2, &lambertian_material);

    camera.samples_per_pixel = 10;
    camera.image_width = 256;

    camera.render(sphere);
}
