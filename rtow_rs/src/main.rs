use std::rc::Rc;
use std::time::Instant;

use camera::Camera;
use color::Color;
use hittable::Hittable;
use material::{Dielectric, Lambertian, Material, Metal};
use rtweekend::{random_double, random_double_range};
use sphere::Sphere;
use vec3::{Point3, Vec3};

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

    let ground_material = Rc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                (a as f64) + 0.9 * random_double(),
                0.2,
                (b as f64) + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;
                if choose_mat < 0.8 {
                    let albedo: Color = Color::random_random() * Color::random_random();
                    sphere_material = Rc::new(Lambertian { albedo });
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal { albedo, fuzz });
                } else {
                    sphere_material = Rc::new(Dielectric {
                        refraction_index: 1.5,
                    });
                }
                world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric {
        refraction_index: 1.5,
    });
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 16;
    camera.max_depth = 8;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(12.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let start = Instant::now();
    camera.render(&world);
    let duration = start.elapsed();

    println!("escape time: {} ms", duration.as_millis());
}
