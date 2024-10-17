#ifndef CAMERA_H
#define CAMERA_H

#include <iostream>
#include "color.h"
#include "hittable.h"
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
                for (int sample = 0; sample < samples_per_pixel; sample++) {
                    ray r = get_ray(i, j);
                    pixel_color += ray_color(r, max_depth, world);
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

    /* Private Camera Variables Here */
    void initialize() {
        // Calculate the image height, and ensure that it's at least 1.
        image_height = int(image_width / aspect_ratio);
        image_height = (image_height < 1) ? 1 : image_height;

        pixel_samples_scale = 1.0 / samples_per_pixel;

        center = point3(0, 0, 0);

        // Determine viewport dimensions.
        auto focal_length = 1.0;
        auto viewport_height = 2.0;
        auto viewport_width = viewport_height * (double(image_width) / image_height);

        // Calculate the vectors across the horizontal and down the vertical
        auto viewport_u = vec3(viewport_width, 0, 0);
        auto viewport_v = vec3(0, -viewport_height, 0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        pixel_delta_u = viewport_u / image_width;
        pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        auto viewport_upper_left = center - vec3(0, 0, focal_length) - viewport_u / 2 - viewport_v / 2;
        pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // std::clog << "\r pixel00_loc is " << pixel00_loc.x() << " " << pixel00_loc.y() << " "
        //           << pixel00_loc.z() << "\n";
        // std::clog << "\r pixel_delta_u is " << pixel_delta_u.x() << " " << pixel_delta_u.y() << " "
        //           << pixel_delta_u.z() << "\n";
        // std::clog << "\r pixel_delta_v is " << pixel_delta_v.x() << " " << pixel_delta_v.y() << " "
        //           << pixel_delta_v.z() << "\n";
    }

    color ray_color(const ray& r, int depth, const hittable& world) const {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if (depth <= 0) {
            return color(0, 0, 0);
        }
        hit_record rec;
        if (world.hit(r, interval(0.001, infinity), rec)) {
            // vec3 direction = random_on_hemisphere(rec.normal);
            vec3 direction = rec.normal + random_unit_vector();
            return 0.5 * ray_color(ray(rec.p, direction), depth - 1, world);
        }

        vec3 unit_direction = unit_vector(r.direction());
        // y[-1, 1] => [0, 1]
        auto a = 0.5 * (unit_direction.y() + 1.0);
        // std::clog << "\r unit vector: " << unit_direction.x() << " " << unit_direction.y() << " "
        //           << unit_direction.z() << ", " << "a is " << a << "\n";
        return (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0);
    }

    ray get_ray(int i, int j) const {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j
        auto offset = sample_square();
        auto pixel_sample =
            pixel00_loc + ((i + offset.x()) * pixel_delta_u) + ((j + offset.y()) * pixel_delta_v);

        auto ray_origin = center;
        auto ray_direction = pixel_sample - ray_origin;
        return ray(ray_origin, ray_direction);
    }

    vec3 sample_square() const {
        // Returns the vector to a random point in the [-0.5, -0.5]-[+0.5, +0.5] unit square.
        return vec3(random_double() - 0.5, random_double() - 0.5, 0);
    }
};

#endif
