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
    let q = in.uv;
    let uv = 0.5 + (q - vec2f(0.5)) * (0.99 + 0.01 * sin(0.2 * globals.time));

    let oricolor = textureSample(screen_texture, texture_sampler, vec2(q.x, q.y));
    var color = vec3f(0);

    color.r = textureSample(screen_texture, texture_sampler, vec2(uv.x + 0.001 * sin(0.4 * globals.time), uv.y)).r;
    color.g = textureSample(screen_texture, texture_sampler, vec2(uv.x + 0.001 * sin(0.4 * globals.time), uv.y)).g;
    color.b = textureSample(screen_texture, texture_sampler, vec2(uv.x + 0.001 * sin(0.4 * globals.time), uv.y)).b;

    color = clamp(color * 0.5 + 0.5 * color * color * 1.2, vec3f(0), vec3f(1.0));

    color *= 0.5 + 0.5 * 16.0 * uv.x * uv.y * (1.0 - uv.x) * (1.0 - uv.y);

    color *= vec3(0.95, 1.1, 0.95);

    color *= 0.9 + 0.1 * sin(10.0 * globals.time - uv.y * 1000.0);

    color *= 0.98 + 0.02 * sin(110.0 * globals.time);

    let blend = 1.5 * pow(length(q - vec2f(0.5)), 4.0);

    color = mix(color, oricolor.rgb, vec3f(blend));

    return vec4f(color.rgb * 1.4, oricolor.a);
}