use camera::Camera;
use color::Color;
use material::{Dielectric, Lambertian};
use sphere::Sphere;
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let mut camera = Camera::new();

    let center: Point3 = Point3::new(0.0, 0.0, -1.0);
    let albedo: Color = Color::random_random() * Color::random_random();
    let lambertian_material = Lambertian { albedo };
    let dielectric_material = Dielectric {
        refraction_index: 1.5,
    };

    let sphere = Sphere::new(center, 0.5, &dielectric_material);

    camera.samples_per_pixel = 50;
    camera.max_depth = 20;
    camera.image_width = 256;

    camera.render(sphere);
}
