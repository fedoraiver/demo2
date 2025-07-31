#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0)
var pattern_texture: texture_2d<f32>;
@group(2) @binding(1)
var pattern_sampler: sampler;


const m: mat2x2<f32> = mat2x2<f32>(
    0.7071, 0.7071,
    -0.7071, 0.7071
);

fn hash(p: vec2f) -> f32 {
    let h = dot(p, vec2f(415.411, 114.514));
    return -1.0 + 2.0 * fract(sin(h) * 911.911911);
}

fn noise(p: vec2f) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(hash(i + vec2f(0.0, 0.0)), hash(i + vec2f(1.0, 0.0)), u.x),
        mix(hash(i + vec2f(0.0, 1.0)), hash(i + vec2f(1.0, 1.0)), u.x),
        u.y
    );
}

fn fbm(q: vec2f) -> f32 {
    var f = 0.0;
    var p = q;
    f += 0.5000 * noise(p); p = m * p * 1.8;
    f += 0.2500 * noise(p); p = m * p * 1.7;
    f += 0.1250 * noise(p); p = m * p * 1.6;
    f += 0.0625 * noise(p);
    return f / 0.9375;
}

fn fbm2(p: vec2f) -> vec2f {
    return vec2f(fbm(p), fbm(vec2f(p.y, p.x)));
}

fn map(p_in: vec2f) -> vec3<f32> {
    var p = p_in * 0.5;
    let t = globals.time * 0.7;

    let f = dot(
        fbm2(1.0 * (0.2 * t + p + fbm2(-0.2 * t + 2.0 * (p + fbm2(4.0 * p))))),
        vec2f(1.0, -1.0)
    );

    let bl = smoothstep(-0.8, 0.8, f);
    let ti = smoothstep(-1.0, 1.0, fbm(p));

    return mix(
        mix(vec3<f32>(0.50, 0.00, 0.00), vec3<f32>(1.00, 0.75, 0.35), ti),
        vec3<f32>(0.00, 0.00, 0.02),
        bl
    );
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let p = in.uv * 2  ;
    let e = 0.005;

    let colc = map(p);
    let gc = dot(colc, vec3<f32>(0.4));

    let cola = map(p + vec2<f32>(e, 0.0));
    let ga = dot(cola, vec3<f32>(0.3));

    let colb = map(p + vec2<f32>(0.0, e));
    let gb = dot(colb, vec3<f32>(0.3));

    let nor = normalize(vec3<f32>(ga - gc, e, gb - gc));

    var col = colc;
    col += vec3<f32>(0.6, 0.2, 0.2) * 8.0 * abs(2.0 * gc - ga - gb);

    let q = in.uv;
    col *= pow(16.0 * q.x * q.y * (1.0 - q.x) * (1.0 - q.y), 0.2);

    return vec4<f32>(
        pow(col.b, 0.75 + 0.15 * cos(globals.time * 0.4)),
        pow(col.r, 1.5) + pow(col.b + col.g, 1.5),
        pow(col.g, 0.75 + 0.15 * sin(globals.time * 0.6)),
        0.25
    );
}
