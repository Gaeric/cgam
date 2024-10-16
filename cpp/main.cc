#include "hittable.h"
#include "hittable_list.h"
#include "rtweekend.h"
#include "sphere.h"

color ray_color(const ray& r, const hittable& world) {
    hit_record rec;
    if (world.hit(r, 0, infinity, rec)) {
        return 0.5 * (rec.normal + color(1, 1, 1));
    }

    vec3 unit_direction = unit_vector(r.direction());
    // y[-1, 1] => [0, 1]
    auto a = 0.5 * (unit_direction.y() + 1.0);
    // std::clog << "\r unit vector: " << unit_direction.x() << " " << unit_direction.y() << " "
    //           << unit_direction.z() << ", " << "a is " << a << "\n";
    return (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0);
}

int main() {
    // Image
    auto aspect_ratio = 16.0 / 9.0;
    int image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    int image_height = int(image_width / aspect_ratio);
    image_height = (image_height < 1) ? 1 : image_height;

    // World
    hittable_list world;
    world.add(make_shared<sphere>(point3(0, 0, -1), 0.5));
    world.add(make_shared<sphere>(point3(0, -100.5, -1), 100));

    // Camera
    // Viewport widths less than one are ok since they are real valued.
    auto focal_length = 1.0;
    auto viewport_height = 2.0;
    auto viewport_width = viewport_height * (double(image_width) / image_height);
    auto camera_center = point3(0, 0, 0);

    // Calculate the vectors across the horizontal and down the vertical
    // viewport edges.
    auto viewport_u = vec3(viewport_width, 0, 0);
    auto viewport_v = vec3(0, -viewport_height, 0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    auto pixel_delta_u = viewport_u / image_width;
    auto pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    auto viewport_upper_left = camera_center - vec3(0, 0, focal_length) - viewport_u / 2 - viewport_v / 2;
    auto pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    //
    // pixel00_loc = viewport_upper_left;
    // std::clog << "\r pixel00_loc is " << pixel00_loc.x() << " " << pixel00_loc.y() << " " <<
    // pixel00_loc.z()
    //           << "\n";

    // std::clog << "\r pixel_delta_u is " << pixel_delta_u.x() << " " << pixel_delta_u.y() << " "
    //           << pixel_delta_u.z() << "\n";
    // std::clog << "\r pixel_delta_v is " << pixel_delta_v.x() << " " << pixel_delta_v.y() << " "
    //           << pixel_delta_v.z() << "\n";

    // Render
    std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

    for (int j = 0; j < image_height; j++) {
        std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
        for (int i = 0; i < image_width; i++) {
            auto pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            auto ray_direction = pixel_center - camera_center;

            // std::clog << "\r pixel_center: " << pixel_center.x() << " " << pixel_center.y() << " "
            //           << pixel_center.z() << "; "
            //           << "ray_direction: " << ray_direction.x() << " " << ray_direction.y() << " "
            //           << ray_direction.z() << "\n";

            ray r(camera_center, ray_direction);

            color pixel_color = ray_color(r, world);
            write_color(std::cout, pixel_color);
        }
    }

    std::clog << "\rDone.           \n";
}
