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

const custom_view = mat4x4<f32>(
  0.0, 0.5, 0.7, 0.0,
  0.0, -0.5, 0.7, 0.0,
  0.7, 0.0, 0.0, 0.0,
  0.0, 0.0, 0.0, 1.0
);

struct CameraUniform {
    view_proj: mat4x4<f32>,
    fovx: f32,
    fovy: f32,
    azimuth: f32,
    elevation: f32,
};
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;

fn screen_to_spheric(screen: vec2<f32>, fov: vec2<f32>, camera_azimuth: f32, camera_elevation: f32) -> vec2<f32> {
  let sx = fov.x * screen.x;
  let sy = fov.y * screen.y;
  let latitude = 90.0 - camera_elevation - sy;
  let latitude_rad = radians(latitude);
  let elevation = 90.0 - latitude;
  let cz = cos(latitude_rad);
  //let azimuth = (camera_azimuth + sx.atan2(cz).to_degrees() + 360.0) % 360.0;
  let azimuth = camera_azimuth + (degrees(atan2(sx, cz)) + 360.0) % 360.0;
  return vec2<f32>(azimuth, elevation);
}

fn project(raycast: vec3<f32>, camera_matrix: mat4x4<f32>) -> vec2<f32> {
  let world = camera_matrix * vec4<f32>(raycast, 0.0);
  let azimuth = atan2(world.z, world.x);
  let elevation = asin(world.y);
  return vec2<f32>(degrees(azimuth)+180.0, degrees(elevation));
}

fn spheric_to_texture(azimuth: f32, elevation: f32) -> vec4<f32> {
  if(azimuth < 0.0) {return vec4<f32>(1.0, 0.0, 0.0, 0.0);}
  if(azimuth > 360.0) {return vec4<f32>(0.0, 0.0, 1.0, 0.0);}

  return textureSample(t_diffuse, s_diffuse, vec2<f32>(azimuth/360.0, elevation/180.0));
}

fn spheric_to_color(azimuth: f32, elevation: f32) -> vec4<f32> {
  var r = (azimuth / 360.0) + (azimuth*10.0)%360.0;
  var g = 0.1;
  var b = ((elevation + 90.0) / (180.0 * 2.0)) + (elevation/abs(elevation))*0.1;
  
  //if elevation > 0.0 {g=0.9;}

  if azimuth < 0.0 || azimuth > 360.0 {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
  }
  if elevation < -90.0 || elevation > 90.0 {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
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
/*
    let spheric = screen_to_spheric(
      uv_corrected,
      vec2<f32>(camera.fovx, camera.fovy),
      camera.azimuth,
      camera.elevation
      );

    let tex_coords = spheric / vec2<f32>(360.0, 180.0);

    var color = textureSample(t_diffuse, s_diffuse, tex_coords);
    return color;
*/

    // Générer un rayon directionnel depuis le centre de la sphère
    let direction = normalize(vec3<f32>(uv_corrected, 1.0));

    // Transformer le rayon dans l'espace de la caméra
    //let world_dir = normalize((camera.view_proj * vec4<f32>(direction, 0.0)).xyz);
    //let world_dir = normalize((custom_view * vec4<f32>(direction, 0.0)).xyz);
    
    let spheric = project(direction, camera.view_proj);
    //var color = textureSample(t_diffuse, s_diffuse, spheric);
    //var color = spheric_to_color(spheric.x, spheric.y); // debug colors
    var color = spheric_to_texture(spheric.x, spheric.y); // debug colors
    return color;
/*
    // Convertir le vecteur directionnel en coordonnées sphériques
    let theta = atan2(world_dir.z, world_dir.x); // Azimut
    let phi = asin(world_dir.y);                // Élévation

    // Convertir les coordonnées sphériques en UV pour échantillonner la texture
    let projection = true;
    var u : f32;
    var v : f32;
    if(projection) {
      u = (theta / (2.0 * PI)) + 0.5; // Normaliser dans [0, 1]
      v = 1.0 - ((phi + (PI / 2.0)) / PI); // Normaliser dans [0, 1]
    }
    else {
      u = coords.x/1920.0;
      v = coords.y/1080.0;
    }


    // debug color
    let debug_color = true;
    var color = textureSample(t_diffuse, s_diffuse, vec2<f32>(u, v));

    if(debug_color) {
      var red = 0.5 * v;
      var blue = 0.5 - (v * 0.5);
      var green = (((u*16)%1) * 0.1) + (((v*32)%1) * 0.1);

      color = vec4f(red, green, blue, 0.0);
    }

    // draw grid lines
    let horizontal_line_size = 0.001;
    let vertical_line_size = 0.001;
    if ( abs(v - 0.5) < horizontal_line_size) { // Equator
      color = vec4f(0.0 , 0.0, 0.0, 0.0); // black
    }
    if ( abs(u - 0.0) < vertical_line_size ) { // North
      color = vec4f(1.0 , 0.0, 0.0, 0.0); // red
    }
    if ( abs(u - 0.5) < vertical_line_size ) { // South
      color = vec4f(1.0 , 1.0, 0.0, 0.0); // yellow 
    }
    if ( abs(u - 0.25) < vertical_line_size ) { // East 
      color = vec4f(0.0 , 0.0, 1.0, 0.0); // blue
    }
    if ( abs(u - 0.75) < vertical_line_size ) { // West
      color = vec4f(0.0 , 1.0, 1.0, 0.0); // cyan
    }

    let show_crosshair = true;
    if(show_crosshair) {
      // draw crosshair
      if((abs(uv.x) < 0.002 /*&& abs(uv.y) < 0.08*/) || // vertical line
          (abs(uv.y) < 0.004 /*&& abs(uv.x) < 0.008*/)) { // horizontal line
        color = vec4f(0.5 , 0.5, 0.5, 0.0);
      }
    }

    //let color = vec4f(((u*36)%1), (32*v)%1, v, 0);
    return color;

    // Échantillonner la texture équirectangulaire
    //return textureSample(t_diffuse, s_diffuse, vec2<f32>(u, v));
  
    //test
    //let toto = vec2<f32>(world_dir)
    //return textureSample(t_diffuse, s_diffuse, uv_corrected);

    //old
    //return textureSample(t_diffuse, s_diffuse, coords);
*/
}

