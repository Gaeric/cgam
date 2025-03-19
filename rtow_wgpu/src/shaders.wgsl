const FLT_MAX: f32 = 3.40282347E+37;

struct Uniforms {
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
};

fn no_intersection() -> Intersection {
    return Intersection(vec3(0.), -1.);
}

struct Sphere {
 center: vec3<f32>,
 radius: f32
};

const OBJECT_COUNT: u32 = 2;
const SCENE: array<Sphere, OBJECT_COUNT> =
  array<Sphere, OBJECT_COUNT>(
    Sphere(vec3(0.0, 0.0, -1.0), 0.5),
    Sphere(vec3(0.0, -100.5, -1.0), 100.0)
);


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
    let t = select(t2, t1, t1 > 0.0);
    if t < 0.0 {
        return no_intersection();
    }

    let p = point_on_ray(ray, t);
    let N = (p - sphere.center) / sphere.radius;

    return Intersection(N, t);
}

@vertex
fn display_vs(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(POSITIONS[in_vertex_index], 1.0);
    return out;
}

@fragment
fn display_fs(in: VertexOutput) -> @location(0) vec4<f32> {
    let origin = vec3(0.0);
    let focus_distance = 1.0;

    let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);

    // Offset and normalize the viewport coordinates of the ray.
    let offset = vec2(f32(uniforms.frame_count % 4) * 0.25 - 0.5,
                      f32((uniforms.frame_count % 16) / 4) * 0.25 - 0.5);

    var uv = (in.clip_position.xy + offset) / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));
    // Normalize the viewport coordinates.
    // let uv = in.clip_position.xy / vec2(f32(uniforms.width - 1u), f32(uniforms.height - 1u));

    // Map `uv` from y-down (normalized) viewport coordinates to camera coordinates.
    // left-top   [-aspect_ratio, 1.0]   right-top    [aspect_ratio, 1.0]
    // left-bottom[-aspect_ratio, -1.0]  right-bottom [aspect_ratio, -1.0]
    let camera_coord_pixel = (2.0 * uv - vec2(1.0)) * vec2(aspect_ratio, -1.0);

    let direction = vec3(camera_coord_pixel, -focus_distance);
    let ray = Ray(origin, direction);

    // let sphere = Sphere(vec3(0.0, 0.0, -1.0), 0.5);
    // if intersect_sphere(ray, sphere) > 0 {
    //     return vec4<f32>(1.0, 0.76, 0.3, 1.0);
    // }

    var closest_hit = Intersection(vec3(0.), FLT_MAX);
    for (var i = 0u; i < OBJECT_COUNT; i += 1u) {
        let sphere = SCENE[i];
        // var sphere = SCENE[i];
        // sphere.radius += sin(f32(uniforms.frame_count) * 0.02) * 0.2;
        let hit = intersect_sphere(ray, sphere);
        if hit.t > 0.0 && hit.t < closest_hit.t {
            closest_hit = hit;
        }
    }

    var radiance_sample: vec3f;
    if closest_hit.t < FLT_MAX {
        radiance_sample = vec3(0.5 * closest_hit.normal + vec3(0.5));
        // return vec4<f32>(1.0, 0.76, 0.03, 1.0) * saturate(1.0 - closest_t);
        // return vec4(saturate(closest_t) * 0.5);
    } else {
        radiance_sample = sky_color(ray);
    }

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

    return vec4<f32>(new_sum / f32(uniforms.frame_count), 1.0);
}
