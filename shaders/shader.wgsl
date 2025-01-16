// Vertex shader

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    //out.tex_coords = model.tex_coords;
    out.tex_coords = vec2<f32>(model.position.x, model.position.y * 0.5) * 0.5 + 0.5;
    //out.clip_position = camera.view_proj * vec4<f32>(model.position.xyz, 1.0); // 2.
    out.clip_position = vec4<f32>(model.position.xy, 0.0, 1.0); 
    return out;
}

// Fragment shader
const PI = 3.141592654;
const two_PI = 2*PI;

struct CameraUniform {
    //width: f32,
    //height: f32,
    //focal_length: f32,
    view_proj: mat4x4<f32>,
    //width: f32,
};

struct CameraSettings {
    width: f32,
    height: f32,
    focal_length: f32,
};

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;
@group(2) @binding(0)
var<uniform> settings: CameraSettings;

fn project(raycast: vec3<f32>, camera_matrix: mat4x4<f32>) -> vec2<f32> {
  let world = camera_matrix * normalize(vec4<f32>(raycast, 0.0));
  let azimuth = (two_PI + atan2(world.x, world.z)) % (two_PI);
  let elevation = abs(acos(world.y));
  return vec2<f32>(degrees(azimuth), degrees(elevation));
}

fn spheric_to_texture(azimuth: f32, elevation: f32) -> vec4<f32> {
  return textureSample(t_diffuse, s_diffuse, vec2<f32>(azimuth/360.0, elevation/180.0));
}

fn spheric_to_color(azimuth: f32, elevation: f32) -> vec4<f32> {
  var r = ((azimuth*10.0)%360.0)/360.0;
  var g = 0.0;
  var b = ((elevation*8.0)%180.0)/180.0;//((elevation + 90.0) / (180.0 * 2.0)) + (elevation/abs(elevation))*0.1;
  
  //if elevation > 0.0 {g=0.9;}

  if azimuth < 0.0 || azimuth > 360.0 {
    return vec4<f32>(0.0, 1.0, 0.0, 0.0);
  }
  if elevation < 0 || elevation > 180.0 {
    return vec4<f32>(1.0, 1.0, 0.0, 0.0);
  }

  return vec4<f32>(r, g, b, 0.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let coords = in.clip_position.xy;

    // Dimensions de l'écran (exemple, à configurer dynamiquement)
    let screen_size = vec2<f32>(1920.0, 1080.0);

    // Normaliser les coordonnées du fragment dans [-1, 1]
    let uv = (coords / screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
    let uv_corrected = vec2<f32>(uv.x, -uv.y); // Inverser Y pour le système d'écran

    // Générer un rayon directionnel depuis le centre de la sphère
    let direction = normalize(vec3<f32>(uv_corrected, settings.focal_length));
    
    let spheric = project(direction, camera.view_proj);
    var color = spheric_to_texture(spheric.x, spheric.y); // debug colors
    return color;
}

