use crate::vec3::{Point3, Vec3};

struct Camera {
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
        }
    }

    pub fn initialize(&mut self) {
        let image_height = self.image_width / self.aspect_ratio as u32;
        if image_height < 1 {
            self.image_height = 1;
        } else {
            self.image_height = image_height;
        }

        self.pixel_sample_scale = 1.0 / (self.samples_per_pixel as f64);
        self.center = self.lookfrom;

    }
}