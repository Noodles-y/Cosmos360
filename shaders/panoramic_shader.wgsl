// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) worldPos: vec3<f32>,
}

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.worldPos = in.position;
    out.tex_coords = in.tex_coords;
    out.clip_position = vec4<f32>(in.position, 1.0);
    return out;
}

const pi:f32 = 3.141592654;
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    
    var screen: vec2<f32>;
    var phi: f32;
    var theta: f32;

    screen = in.tex_coords;
    phi = screen.x * pi;
    theta = screen.y * pi/2.0;

    var dir: vec3<f32>;
    dir.x = cos(theta) * sin(phi);
    dir.y = sin(theta);
    dir.z = cos(theta) * cos(phi);

    var u: f32 = 0.5 + atan2(dir.z, dir.x) / (2*pi);
    var v: f32 = 0.5 - asin(dir.y) / pi;

    var tex_coord = vec2<f32>(u, v);

    return textureSampleLevel(t_diffuse, s_diffuse, tex_coord, 0.0);
}

