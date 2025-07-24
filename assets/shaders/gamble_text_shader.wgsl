#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0)
var pattern_texture: texture_2d<f32>;
@group(2) @binding(1)
var pattern_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;

    // 曲面畸变
    let center = vec2<f32>(0.5, 0.5);
    let offset = uv - center;
    let distortion = 0.3;
    uv = center + offset * (1.0 + distortion * length(offset) * length(offset));

    // 扫描线动画
    let scan_speed = -40.0;
    let scan = 0.2 * sin(uv.y * 100.0 - uv.x * 200.0 + globals.time * scan_speed);

    // 采样颜色（带 alpha）
    let rgba = textureSample(pattern_texture, pattern_sampler, uv);

    // RGB分离模拟
    let r = textureSample(pattern_texture, pattern_sampler, uv + vec2<f32>(0.005, 0.005)).r;
    let g = textureSample(pattern_texture, pattern_sampler, uv + vec2<f32>(0.005, 0.005)).g;
    let b = textureSample(pattern_texture, pattern_sampler, uv + vec2<f32>(-0.005, -0.005)).b;
    let base_color = vec3<f32>(r, g, b);

    // 扫描线 + vignette
    let brightness = 1.0 - scan;
    let vignette = smoothstep(0.8, 0.2, length(offset));
    let final_color = base_color * brightness * vignette;

    // 保留透明度（防止黑边）
    return vec4<f32>(final_color, rgba.a);
}
