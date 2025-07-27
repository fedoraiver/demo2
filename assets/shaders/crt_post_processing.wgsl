#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::globals::Globals


struct PostProcessSettings {
    intensity: f32,
    band_mult: f32,
    cell_mult: f32,
    brightness: f32,
};

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;
@group(0) @binding(3) var<uniform> globals: Globals;


@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let banding = abs(sin((in.position.y - globals.time * 100.0) * settings.band_mult));
    let cell_ndx = u32(in.position.x * settings.cell_mult) % 3;
    var cell_color = vec3f(0.0);
    cell_color[cell_ndx] = 1.0 + settings.brightness;
    let effect = mix(vec3f(1.0), banding * cell_color, settings.intensity);
    let color = textureSample(screen_texture, texture_sampler, in.uv);
    return vec4f(color.rgb * effect, color.a);
}