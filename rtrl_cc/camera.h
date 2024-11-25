#ifndef CAMERA_H
#define CAMERA_H

#include <iostream>
#include "color.h"
#include "hittable.h"
#include "material.h"
#include "rtweekend.h"
#include "vec3.h"

class camera {
   public:
    // Ratio of image width over height
    double aspect_ratio = 1.0;
    // Rendered image width in pixel count
    int image_width = 100;
    // Count of random samples for each pixel
    int samples_per_pixel = 10;
    // Maxmium number of ray bounces into scene
    int max_depth = 10;
    // Scene background color
    color background;

    // Vertical view angle (field of view)
    double vfov = 90;
    // Point camera is looking from
    point3 lookfrom = point3(0, 0, 0);
    // Point camera is looking at
    point3 lookat = point3(0, 0, -1);
    // Camera-relative "up" direction
    vec3 vup = vec3(0, 1, 0);

    // Variation angle of rays through each pixel
    double defocus_angle = 0;
    // Distance from camera lookfrom point to plane of perfect focus
    double focus_dist = 10;

    /* Public Camera Parameters Here */
    void render(const hittable& world) {
        initialize();

        // std::clog << "\r render pixel00_loc is " << pixel00_loc.x() << " " << pixel00_loc.y() << " "
        //           << pixel00_loc.z() << "\n";

        // std::clog << "\r render pixel_delta_u is " << pixel_delta_u.x() << " " << pixel_delta_u.y() << " "
        //           << pixel_delta_u.z() << "\n";
        // std::clog << "\r render pixel_delta_v is " << pixel_delta_v.x() << " " << pixel_delta_v.y() << " "
        //           << pixel_delta_v.z() << "\n";

        // Render
        std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

        for (int j = 0; j < image_height; j++) {
            std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
            for (int i = 0; i < image_width; i++) {
                color pixel_color(0, 0, 0);
                for (int s_j = 0; s_j < sqrt_spp; s_j++) {
                    for (int s_i = 0; s_i < sqrt_spp; s_i++) {
                        ray r = get_ray(i, j, s_i, s_j);
                        pixel_color += ray_color(r, max_depth, world);
                    }
                }
                write_color(std::cout, pixel_samples_scale * pixel_color);
            }
        }

        std::clog << "\rDone.           \n";
    }

   private:
    // Render image height
    int image_height;
    // Color scale factor for a sum of pixel samples
    double pixel_samples_scale;
    // Camera center
    point3 center;
    // Location of pixel 0, 0
    point3 pixel00_loc;
    // Offset to pixel to the right
    vec3 pixel_delta_u;
    // Offset to pixel below
    vec3 pixel_delta_v;
    // Camera frame basis vectors
    vec3 u, v, w;
    // Defocus disk horizontal radius
    vec3 defocus_disk_u;
    // Defocus disk vertical radius
    vec3 defocus_disk_v;
    // Square root of number of samples per pixel
    int sqrt_spp;
    //  1 / sqrt_spp
    double recip_sqrt_spp;

    /* Private Camera Variables Here */
    void initialize() {
        // Calculate the image height, and ensure that it's at least 1.
        image_height = int(image_width / aspect_ratio);
        image_height = (image_height < 1) ? 1 : image_height;

        sqrt_spp = int(std::sqrt(samples_per_pixel));
        pixel_samples_scale = 1.0 / (sqrt_spp * sqrt_spp);
        recip_sqrt_spp = 1.0 / sqrt_spp;

        center = lookfrom;

        // Determine viewport dimensions.
        auto theta = degress_to_radians(vfov);
        auto h = std::tan(theta / 2);
        auto viewport_height = 2.0 * h * focus_dist;
        auto viewport_width = viewport_height * (double(image_width) / image_height);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame;
        // since we only have the exact OP vector, we cannot describe the rotation around OP(roll).
        w = unit_vector(lookfrom - lookat);
        u = unit_vector(cross(vup, w));
        v = cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical
        // Vector across viewport horizontal edge
        auto viewport_u = viewport_width * u;
        // Vector down viewport vectical edge
        auto viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        pixel_delta_u = viewport_u / image_width;
        pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        auto viewport_upper_left = center - (focus_dist * w) - viewport_u / 2 - viewport_v / 2;
        pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // std::clog << "\r pixel00_loc is " << pixel00_loc.x() << " " << pixel00_loc.y() << " "
        //           << pixel00_loc.z() << "\n";
        // std::clog << "\r pixel_delta_u is " << pixel_delta_u.x() << " " << pixel_delta_u.y() << " "
        //           << pixel_delta_u.z() << "\n";
        // std::clog << "\r pixel_delta_v is " << pixel_delta_v.x() << " " << pixel_delta_v.y() << " "
        //           << pixel_delta_v.z() << "\n";
        auto defocus_radius = focus_dist * std::tan(degress_to_radians(defocus_angle / 2));
        defocus_disk_u = u * defocus_radius;
        defocus_disk_v = v * defocus_radius;
    }

    color ray_color(const ray& r, int depth, const hittable& world) const {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if (depth <= 0) {
            return color(0, 0, 0);
        }
        hit_record rec;
        if (!world.hit(r, interval(0.001, infinity), rec)) {
            return background;
        }

        ray scattered;
        color attenuation;
        color color_from_emission = rec.mat->emitted(rec.u, rec.v, rec.p);

        if (!rec.mat->scatter(r, rec, attenuation, scattered)) {
            return color_from_emission;
        }

        double scattering_pdf = rec.mat->scattering_pdf(r, rec, scattered);
        // double pdf_value = scattering_pdf;
        double pdf_value = 1 / (2 * pi);

        color color_from_scatter =
            (attenuation * scattering_pdf * ray_color(scattered, depth - 1, world)) / pdf_value;

        return color_from_emission + color_from_scatter;
    }

    ray get_ray(int i, int j, int s_i, int s_j) const {
        // Construct a camera ray originating from the defocus disk and directed at randomly sampled
        // point around the pixel location i, j
        // For stratified sample square s_i, s_j
        auto offset = sample_square_stratified(s_i, s_j);
        auto pixel_sample =
            pixel00_loc + ((i + offset.x()) * pixel_delta_u) + ((j + offset.y()) * pixel_delta_v);

        auto ray_origin = (defocus_angle <= 0) ? center : defocus_disk_sample();
        auto ray_direction = pixel_sample - ray_origin;
        auto ray_time = random_double();
        return ray(ray_origin, ray_direction, ray_time);
    }

    vec3 sample_square_stratified(int s_i, int s_j) const {
        // Returns the vector to a random point in the square sub-pixel specified by grid
        // indices s_i and s_j, for an idealized unit square pixel [-.5, -.5] to [+.5, +.5].
        auto px = ((s_i + random_double()) * recip_sqrt_spp) - 0.5;
        auto py = ((s_j + random_double()) * recip_sqrt_spp) - 0.5;
        return vec3(px, py, 0);
    }

    vec3 sample_square() const {
        // Returns the vector to a random point in the [-0.5, -0.5]-[+0.5, +0.5] unit square.
        return vec3(random_double() - 0.5, random_double() - 0.5, 0);
    }

    point3 defocus_disk_sample() const {
        // Returns a random point in the camera defocus disk.
        auto p = random_in_unit_disk();
        return center + (p[0] * defocus_disk_u) + (p[1] * defocus_disk_v);
    }
};

#endif
