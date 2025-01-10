struct Uniforms {
 width: u32,
 height: u32,
};

struct Ray {
 origin: vec3<f32>,
 direction: vec3<f32>,
}


@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

const POSITIONS: array<vec3<f32>, 6> =
    array<vec3<f32>, 6>(
    vec3<f32>(-1.0,  1.0, 0.0),
    vec3<f32>(-1.0, -1.0, 0.0),
    vec3<f32>( 1.0,  1.0, 0.0),
    vec3<f32>( 1.0,  1.0, 0.0),
    vec3<f32>(-1.0, -1.0, 0.0),
    vec3<f32>( 1.0, -1.0, 0.0),
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

    // Normalize the viewport coordinates.
    let uv = in.clip_position.xy / vec2(f32(uniforms.width - 1u), f32(uniforms.height - 1u));

    // Map `uv` from y-down (normalized) viewport coordinates to camera coordinates.
    // left-top   [-aspect_ratio, 1.0]   right-top    [aspect_ratio, 1.0]
    // left-bottom[-aspect_ratio, -1.0]  right-bottom [aspect_ratio, -1.0]
    let camera_coord_pixel = (2.0 * uv - vec2(1.0)) * vec2(aspect_ratio, -1.0);

    let direction = vec3(camera_coord_pixel, -focus_distance);
    let ray = Ray(origin, direction);

    return vec4<f32>(sky_color(ray), 1.0);
}
