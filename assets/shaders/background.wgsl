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

    let scan_speed = -(6.0 + 2.0 * sin(globals.time * 0.01));
    let scan = 0.6 * sin((uv.y) * 120.0 + globals.time * scan_speed);

    let rgba = textureSample(pattern_texture, pattern_sampler, uv);

    let r = rgba.r ;
    let g = rgba.g ;
    let b = rgba.b ;
    let base_color = vec3<f32>(r, g, b);

    let brightness = 1.0 - scan;
    let vignette = smoothstep(1.3, 0.0, length(offset));
    let final_color = base_color * brightness * vignette;

    return vec4<f32>(final_color, rgba.a);
}
