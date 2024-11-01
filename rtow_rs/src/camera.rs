use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use core::time;
use std::{io::Write, thread::sleep};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub delay: u64,

    /// Render image height
    image_height: u32,
    /// Color scale factor for a sum of pixel samples
    pixel_sample_scale: f64,
    center: Point3,
    // location of pixel, u->right, v->below
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    // Defocus disk raidus
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,

            image_height: 0,
            pixel_sample_scale: 0.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),

            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),

            delay: 0,
        }
    }

    fn initialize(&mut self) {
        let image_height = self.image_width / self.aspect_ratio as u32;
        if image_height < 1 {
            self.image_height = 1;
        } else {
            self.image_height = image_height;
        }

        self.pixel_sample_scale = 1.0 / (self.samples_per_pixel as f64);
        self.center = self.lookfrom;
    }

    fn ray_color<T: Hittable>(r: &mut Ray, depth: i32, world: T) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec: HitRecord = Default::default();
        let interval = Interval::new(0.001, std::f64::INFINITY);

        if world.hit(r, interval, &mut rec) {
            let mut scattered: Ray = Default::default();
            let mut attenuation: Color = Default::default();
            if let Some(mat) = rec.mat {
                if mat.scatter(r, &mut rec, &mut attenuation, &mut scattered) {
                    return attenuation * Self::ray_color(&mut scattered, depth - 1, world);
                }
            }
        }

        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render<T: Hittable>(&mut self, world: T) {
        const IMAGE_WIDTH: i32 = 256;
        const IMAGE_HEIGHT: i32 = 256;
        let ms = time::Duration::from_millis(self.delay);

        self.initialize();

        println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

        for j in 0..IMAGE_HEIGHT {
            eprint!("\rScanlines remaining: {}", IMAGE_HEIGHT - j);
            std::io::stderr().flush().unwrap();
            sleep(ms);
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
                    let g = j as f64 / (IMAGE_WIDTH - 1) as f64;
                    let b = 0.0 as f64;
                    pixel_color += Color::new(r, g, b);
                }

                let pixel_color = pixel_color * self.pixel_sample_scale;

                write_color(&pixel_color);
            }
        }

        eprintln!("\rDone.        ");
    }
}
