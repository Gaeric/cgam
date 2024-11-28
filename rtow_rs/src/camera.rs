use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableCollection;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::degress_to_radians;
use crate::vec3::{Point3, Vec3};

use core::time;
use std::fs::File;
use std::sync::Arc;
use std::{io::Write, thread::sleep};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
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

        // Determine viewport dimensions.
        let theta = degress_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame;
        // since we only have the exact OP vector, we cannot describe the rotation around OP(roll).
        self.w = (self.lookfrom - self.lookat).unit();
        self.u = self.vup.cross(self.w).unit();
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical
        // Vector across viewport horizontal edge
        let viewport_u = viewport_width * self.u;
        // Vector down viewport vectical edge
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (degress_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color<T: HittableCollection>(r: &mut Ray, depth: i32, world: &T) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec: HitRecord = Default::default();
        let interval = Interval::new(0.001, std::f64::INFINITY);

        if world.hit(r, interval, &mut rec) {
            let mut scattered: Ray = Default::default();
            let mut attenuation: Color = Default::default();
            if let Some(ref mat) = rec.mat {
                let mat_clone = Arc::clone(mat);
                if mat_clone.scatter(r, &mut rec, &mut attenuation, &mut scattered) {
                    return attenuation * Self::ray_color(&mut scattered, depth - 1, world);
                }
            }
        }

        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-0.5, -0.5]-[+0.5, +0.5] unit square.
        Vec3::random_random() - Vec3::new(0.5, 0.5, 0.0)
    }

    pub fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        // eprintln!("offset {:#?}, pixel_sample {:#?}\n", offset, pixel_sample);
        let ray_origin: Vec3;
        if self.defocus_angle <= 0.0 {
            ray_origin = self.center;
        } else {
            ray_origin = self.defocus_disk_sample();
        }

        // if self.defocus_angle <= 0.0
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn render<T: HittableCollection>(&mut self, world: &T) {
        let ms = time::Duration::from_millis(self.delay);

        self.initialize();

        let mut file = File::create("output.ppm").unwrap();

        let _ = writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_width {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            std::io::stderr().flush().unwrap();
            sleep(ms);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let mut r = self.get_ray(i as i32, j as i32);
                    let sample_color = Self::ray_color(&mut r, self.max_depth, world);
                    // eprintln!("ray {:#?}, sample color: {:#?}", r, sample_color);

                    pixel_color += sample_color;
                }

                let pixel_color = pixel_color * self.pixel_sample_scale;

                write_color(&mut file, &pixel_color);
            }
        }

        eprintln!("\rDone.        ");
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        return self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v);
    }
}
