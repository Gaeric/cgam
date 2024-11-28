use std::{sync::Arc, time::Instant};

use camera::Camera;
use color::Color;
use hittable::Hittable;
use material::{Dielectric, Lambertian};
use sphere::Sphere;
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let mut camera = Camera::new();
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let center: Point3 = Point3::new(0.0, 0.0, -1.0);
    let albedo: Color = Color::random_random() * Color::random_random();
    let lambertian_material = Arc::new(Lambertian { albedo });
    let dielectric_material = Arc::new(Dielectric {
        refraction_index: 1.5,
    });

    camera.samples_per_pixel = 50;
    camera.max_depth = 20;
    camera.image_width = 256;

    let start = Instant::now();

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        lambertian_material,
    )));
    world.push(Box::new(Sphere::new(center, 0.5, dielectric_material)));

    camera.render(&world);

    let duration = start.elapsed();

    println!("escape time: {} ms", duration.as_millis());
}
