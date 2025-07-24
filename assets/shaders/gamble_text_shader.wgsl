#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var pattern_texture: texture_2d<f32>;
@group(2) @binding(1)
var pattern_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(pattern_texture, pattern_sampler, in.uv);
    return color;
}
