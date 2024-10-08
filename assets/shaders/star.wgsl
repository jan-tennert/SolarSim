#import bevy_pbr::forward_io::VertexOutput

// Uniform for the material color (you can set this to any color for your sun)
@group(2) @binding(0) var<uniform> material_color: vec4<f32>;

// Texture and sampler
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;

// Uniforms for controlling glow effect
@group(2) @binding(3) var<uniform> glow_intensity: f32;
@group(2) @binding(4) var<uniform> glow_radius: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Get the UV coordinates from the mesh
    let uv = mesh.uv;

    // Calculate distance from the center of the UV (center of the sun)
    let dist_from_center = length(uv - vec2<f32>(0.5, 0.5));

    // Create a smooth glow that fades as distance from center increases
    let glow_factor = glow_intensity / (dist_from_center * glow_radius + 1.0);

    // Apply the texture and the glow effect to the material color
    let texture_color = textureSample(material_color_texture, material_color_sampler, uv);

    // Multiply the texture color by the glow factor and the material color
    return material_color * texture_color * vec4<f32>(glow_factor, glow_factor, glow_factor, 1.0);
}