#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

struct PostProcessSettings {
    intensity: f32,
    band_mult: f32,
    cell_mult: f32,
    brightness: f32,
};

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let rgba = textureSample(screen_texture, texture_sampler, in.uv);
    return vec4(0.0, rgba.g, rgba.b, 1.0);
}