const FLT_MAX: f32 = 3.40282347E+37;
const EPSILON: f32 = 1e-3;
const TWO_PI: f32 = 6.2831853;

struct CameraUniforms {
 origin: vec3f,
 u: vec3f,
 v: vec3f,
 w: vec3f,
}

struct Uniforms {
 camera: CameraUniforms,
 width: u32,
 height: u32,
 frame_count: u32,
};

struct Ray {
 origin: vec3<f32>,
 direction: vec3<f32>,
};

struct Intersection {
 normal: vec3f,
 t: f32,
 material_index: u32,
};

struct Material {
 color: vec3f,
 specular_or_ior: f32,
}

fn no_intersection() -> Intersection {
    return Intersection(vec3(0.), -1.0, 0);
}

fn is_intersection_valid(hit: Intersection) -> bool {
    return hit.t > 0.0;
}

struct Sphere {
 center: vec3<f32>,
 radius: f32,
 material_index: u32,
};

const OBJECT_COUNT: u32 = 4;
const SCENE: array<Sphere, OBJECT_COUNT> =
  array<Sphere, OBJECT_COUNT>(
    Sphere(vec3(-1.1, 0.5, 0.0), 0.5, 0),
    Sphere(vec3(0.0, 0.5, 0.0), 0.5, 1),
    Sphere(vec3(1.1, 0.5, 0.0), 0.5, 3),

    // Gound
    Sphere(vec3(0.0, -2e2 - EPSILON, 0.0), 2e2, 2),
);

const MATERIAL_COUNT: u32 = 4;
const MATERIALS: array<Material, MATERIAL_COUNT> =
  array<Material, MATERIAL_COUNT>(
    Material(vec3(0.7, 0.5, 0.5), 1),
    Material(vec3(0.5, 0.5, 0.9), 0),
    Material(vec3(0.7, 0.9, 0.2), 0),
    Material(vec3(1.0, 1.0, 1.0), -1.5),
);

const MAX_PATH_LENGTH: u32 = 13u;

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var radiance_samples_old: texture_2d<f32>;
@group(0) @binding(2) var radiance_samples_new: texture_storage_2d<rgba32float, write>;


const POSITIONS: array<vec3<f32>, 6> =
    array<vec3<f32>, 6>(
    vec3<f32>(-1.0, 1.0, 0.0),
    vec3<f32>(-1.0, -1.0, 0.0),
    vec3<f32>(1.0, 1.0, 0.0),
    vec3<f32>(1.0, 1.0, 0.0),
    vec3<f32>(-1.0, -1.0, 0.0),
    vec3<f32>(1.0, -1.0, 0.0),
);

const WIDTH: u32 = 800u;
const HEIGHT: u32 = 600u;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

struct Rng {
 state: u32,
};

var<private> rng: Rng;

fn init_rng(pixel: vec2u) {
    // Seed the PRNG using the scalar index of the pixel and the current frame count
    let seed = (pixel.x + pixel.y * uniforms.width) ^ jenkins_hash(uniforms.frame_count);
    rng.state = jenkins_hash(seed);
}

// A slightly modified version of the "One-at-a-Time Hash" function by Bob Jenkins.
// See https://www.burtleburtle.net/bob/hash/doobs.html
fn jenkins_hash(i: u32) -> u32 {
    var x = i;
    x += x << 10u;
    x ^= x >> 6u;
    x += x << 3u;
    x ^= x >> 11u;
    x += x << 15u;
    return x;
}

// The 32-bit "xor" function from Marsaglia G., "Xorshift RNGs", Section 3.
fn xorshift32() -> u32 {
    var x = rng.state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    rng.state = x;
    return x;
}

// Returns a random float in the range [0..1]. This sets the floating point exponent to zero and
// sets the most significant 23 bits of a random 32-bit unsigend integer as the mantissa. That
// generates a number in the range [1, 1.9999999], which is the mapped to [0, 0.9999999] by
// subtraction. See Ray Tracing Gems II, Section 14.3.4
fn rand_f32() -> f32 {
    return bitcast<f32>(0x3f800000u | (xorshift32() >> 9u)) - 1.0;
}

// Uniformly sample a unit sphere centered at the origin
fn sample_sphere() -> vec3f {
    let r0 = rand_f32();
    let r1 = rand_f32();

  // Map r0 to [-1, 1]
    let y = 1.0 - 2.0 * r0;

  // Compute the projected radius on the xz-plane using Pythagorean theorem
    let xz_r = sqrt(1.0 - y * y);

    let phi = TWO_PI * r1;
    return vec3(xz_r * cos(phi), y, xz_r * sin(phi));
}

struct Scatter {
 attenuation: vec3f,
 ray: Ray,
};

fn sample_lambertian(normal: vec3f) -> vec3f {
    return normal + sample_sphere() * (1.0 - EPSILON);
}

fn scatter(input_ray: Ray, hit: Intersection, material: Material) -> Scatter {
    var scattered: vec3f;
    let incident = normalize(input_ray.direction);
    if (material.specular_or_ior == 0.0) {
      scattered = sample_lambertian(hit.normal);
    } else {
      let ior = abs(material.specular_or_ior);
      scattered = refract(incident, hit.normal, ior);
      if (material.specular_or_ior > 0.0 || dot(scattered, scattered) == 0.0) {
        scattered = reflect(incident, hit.normal);
      }
    }

    let output_ray = Ray(point_on_ray(input_ray, hit.t), scattered);
    let attenuation = material.color;
    return Scatter(attenuation, output_ray);
}

fn intersect_scene(ray: Ray) -> Intersection {
    var closest_hit = no_intersection();
    closest_hit.t = FLT_MAX;
    for (var i = 0u; i < OBJECT_COUNT; i += 1u) {
        let sphere = SCENE[i];
        let hit = intersect_sphere(ray, sphere);
        if hit.t > 0.0 && hit.t < closest_hit.t {
            closest_hit = hit;
        }
    }

    if closest_hit.t < FLT_MAX {
        return closest_hit;
    }

    return no_intersection();
}

fn sky_color(ray: Ray) -> vec3<f32> {
    // maybe be some mistake
    let t = 0.5 * (normalize(ray.direction).y + 1.0);
    return (1.0 - t) * vec3(1.0) + t * vec3(0.3, 0.5, 1.0);
}

fn point_on_ray(ray: Ray, t: f32) -> vec3<f32> {
    return ray.origin + t * ray.direction;
}

fn intersect_sphere(ray: Ray, sphere: Sphere) -> Intersection {
    let v = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let d = dot(v, ray.direction);
    let c = dot(v, v) - sphere.radius * sphere.radius;

    let delta = d * d - a * c;

    if delta < 0.0 {
        return no_intersection();
    }

    let sqrt_delta = sqrt(delta);
    let recip_a = 1.0 / a;
    let md = -d;

    let t1 = (md - sqrt_delta) * recip_a;
    let t2 = (md + sqrt_delta) * recip_a;
    let t = select(t2, t1, t1 > EPSILON);
    if t < EPSILON {
        return no_intersection();
    }

    let p = point_on_ray(ray, t);
    let N = (p - sphere.center) / sphere.radius;

    return Intersection(N, t, sphere.material_index);
}

@vertex
fn display_vs(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(POSITIONS[in_vertex_index], 1.0);
    return out;
}

@fragment
fn display_fs(in: VertexOutput) -> @location(0) vec4<f32> {
    let origin = uniforms.camera.origin;
    let focus_distance = 1.0;

    let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);

    // Offset and normalize the viewport coordinates of the ray.
    init_rng(vec2u(in.clip_position.xy));
    let offset = vec2(rand_f32() - 0.5, rand_f32() - 0.5);
    // let offset = vec2(f32(uniforms.frame_count % 4) * 0.25 - 0.5,
    //                   f32((uniforms.frame_count % 16) / 4) * 0.25 - 0.5);
    var uv = (in.clip_position.xy + offset) / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));
    // Normalize the viewport coordinates.
    // let uv = in.clip_position.xy / vec2(f32(uniforms.width - 1u), f32(uniforms.height - 1u));

    // Map `uv` from y-down (normalized) viewport coordinates to camera coordinates.
    // left-top   [-aspect_ratio, 1.0]   right-top    [aspect_ratio, 1.0]
    // left-bottom[-aspect_ratio, -1.0]  right-bottom [aspect_ratio, -1.0]
    let camera_coord_pixel = (2.0 * uv - vec2(1.0)) * vec2(aspect_ratio, -1.0);

    let camera_rotation = mat3x3(uniforms.camera.u, uniforms.camera.v, uniforms.camera.w);
    // translate the direction vector from camera space to world space
    let direction = camera_rotation * vec3(camera_coord_pixel, -focus_distance);

    var ray = Ray(origin, direction);
    var radiance_sample: vec3f = vec3(0.0);
    var throughput = vec3f(1.0);
    var path_length = 0u;

    while path_length < MAX_PATH_LENGTH {
        let hit = intersect_scene(ray);
        if !is_intersection_valid(hit) {
            // If no intersection was found, return the color of the sky and terminate the path.
            radiance_sample += throughput * sky_color(ray);
            break;
        }

        let material = MATERIALS[hit.material_index];
        let scattered = scatter(ray, hit, material);
        throughput *= scattered.attenuation;
        ray = scattered.ray;
        path_length += 1u;
    }

    // let sphere = Sphere(vec3(0.0, 0.0, -1.0), 0.5);
    // if intersect_sphere(ray, sphere) > 0 {
    //     return vec4<f32>(1.0, 0.76, 0.3, 1.0);
    // }
    // let hit = intersect_scene(ray);
    // if is_intersection_valid(hit) {
    //     radiance_sample = vec3(0.5 * hit.normal + vec3(0.5));
    //     // return vec4<f32>(1.0, 0.76, 0.03, 1.0) * saturate(1.0 - closest_t);
    //     // return vec4(saturate(closest_t) * 0.5);
    // } else {
    //     radiance_sample = sky_color(ray);
    // }

    // Fetch the old sum of samples
    var old_sum: vec3f;
    if uniforms.frame_count > 1 {
        old_sum = textureLoad(radiance_samples_old, vec2u(in.clip_position.xy), 0).xyz;
    } else {
        old_sum = vec3(0.0);
    }

    // Compute and store the new sum.
    let new_sum = radiance_sample + old_sum;
    textureStore(radiance_samples_new, vec2u(in.clip_position.xy), vec4(new_sum, 0.0));

    // Display the average after gamma correction (gamma = 2.2)
    let color = new_sum / f32(uniforms.frame_count);
    return vec4(pow(color, vec3(1.0 / 2.2)), 1.0);
}
