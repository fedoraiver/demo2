#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0)
var pattern_texture: texture_2d<f32>;
@group(2) @binding(1)
var pattern_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;

    let center = vec2<f32>(0.5, 0.0);
    let offset = uv - center;
    let distortion = 1.0;
    uv = center + offset * (1.0 + distortion * length(offset) * length(offset));

    let scan_speed = -4.0;
    let scan = 0.6 * sin((uv.y) * 160.0 + globals.time * scan_speed);

    let rgba = textureSample(pattern_texture, pattern_sampler, uv);

    let r = rgba.r * abs(sin(uv.x * 200.0 + globals.time * 0.3));
    let g = rgba.g * abs(sin(uv.x * 20.0 + globals.time * 0.2));
    let b = rgba.b * abs(sin(uv.x * -200.0 + globals.time * 0.3));
    let base_color = vec3<f32>(mix(r, b, sin(globals.time * 1)), g, mix(b, r, cos(globals.time * 1)));

    let brightness = 1.0 - scan;
    let vignette = smoothstep(1.3, 0.0, length(offset));
    let final_color = base_color * brightness * vignette;

    return vec4<f32>(final_color, rgba.a);
}
