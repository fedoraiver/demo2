#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0)
var pattern_texture: texture_2d<f32>;
@group(2) @binding(1)
var pattern_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;

    let center = vec2<f32>(0.5, 0.5);
    let offset = uv - center;
    let distortion = 0.3;
    uv = center + offset * (1.0 + distortion * length(offset) * length(offset));

    let scan_speed = -40.0;
    let scan = 0.4 * (sin(uv.y * 100.0 - uv.x * 200.0 + globals.time * scan_speed) + 1.0) / 2.0;

    let rgba = textureSample(pattern_texture, pattern_sampler, uv + vec2<f32>(0.02 * sin(globals.time), 0.02 * sin(globals.time)));
    let a = rgba.a;

    let r = textureSample(pattern_texture, pattern_sampler, uv + vec2<f32>(0.02 * sin(globals.time), 0.02 * sin(globals.time))).r * a;
    let g = textureSample(pattern_texture, pattern_sampler, uv + vec2<f32>(0.02 * sin(globals.time), 0.02 * sin(globals.time))).g * a;
    let b = textureSample(pattern_texture, pattern_sampler, uv + vec2<f32>(0.02 * sin(globals.time), 0.02 * sin(globals.time))).b * a;
    let base_color = vec3<f32>(r, mix(g, r, 0.5 * (1 + sin(globals.time))), b + r);

    let brightness = 1.0 - scan;
    let vignette = smoothstep(1.0, 0.0, length(offset));
    let final_color = base_color * brightness * vignette;

    return vec4<f32>(final_color, rgba.a * (0.85 + 0.15 * cos(globals.time * 2.0)));
}
