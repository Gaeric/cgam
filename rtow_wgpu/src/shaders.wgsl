const POSITIONS: array<vec3<f32>, 3> =
    array<vec3<f32>, 3>(
    vec3<f32>(-0.5, -0.5, 0.0),
    vec3<f32>(0.5, -0.5, 0.0),
    vec3<f32>(0.0, 0.5, 0.0)
);

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn display_vs(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(POSITIONS[in_vertex_index], 1.0);
    return out;
}

@fragment
// fn display_fs(in: VertexOutput) -> @location(0) vec4<f32> {
fn display_fs() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
