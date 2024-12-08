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
    out.tex_coords = model.position.xy * 0.5 + 0.5;
    //out.clip_position = camera.view_proj * vec4<f32>(model.position.xyz, 1.0); // 2.
    out.clip_position = vec4<f32>(model.position.xyz, 1.0); 
    return out;
}

// Fragment shader
const PI = 3.141592654;

struct CameraUniform {
    view_proj: mat4x4<f32>,
    //angular: f32,
    //radial: f32,
};
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //var coords = in.tex_coords;
    var coords = in.clip_position.xy;

    //coords.y -= camera.radial / 180.0;
    //coords.x += camera.angular / 360.0;

    //if(coords.y > 1.0 || coords.y < 0.0) {
    //  coords.x = (-coords.x) %1.0;
    //}
    //else {
    //  coords.x = coords.x %1.0;
    //}

    // Dimensions de l'écran (exemple, à configurer dynamiquement)
    let screen_size = vec2<f32>(1920.0, 1080.0);

    // Normaliser les coordonnées du fragment dans [-1, 1]
    let uv = (coords / screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
    //let uv = coords * 2.0 - vec2<f32>(1.0, 1.0);

    //let ratio = vec2<f32>(2.0, 2.0);
    //let uv = coords * ratio - (ratio.x/2.0);
    

    let uv_corrected = vec2<f32>(uv.x, -uv.y); // Inverser Y pour le système d'écran

    // Générer un rayon directionnel depuis le centre de la sphère
    let direction = normalize(vec3<f32>(uv_corrected, 1.0));

    // Transformer le rayon dans l'espace de la caméra
    let world_dir = normalize((camera.view_proj * vec4<f32>(direction, 0.0)).xyz);

    // Convertir le vecteur directionnel en coordonnées sphériques
    let theta = atan2(world_dir.z, world_dir.x); // Azimut
    let phi = asin(world_dir.y);                // Élévation

    // Convertir les coordonnées sphériques en UV pour échantillonner la texture
    let u = (theta / (2.0 * 3.14159265359)) + 0.5; // Normaliser dans [0, 1]
    let v = 1.0 - ((phi + (3.14159265359 / 2.0)) / 3.14159265359); // Normaliser dans [0, 1]

    // Échantillonner la texture équirectangulaire
    return textureSample(t_diffuse, s_diffuse, vec2<f32>(u, v));
  
    //test
    //let toto = vec2<f32>(world_dir)
    //return textureSample(t_diffuse, s_diffuse, uv_corrected);

    //old
    //return textureSample(t_diffuse, s_diffuse, coords);
}

