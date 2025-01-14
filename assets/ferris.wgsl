#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_pbr::utils::hsv2rgb

@group(2) @binding(0) var<uniform> rotation: u32;
@group(2) @binding(1) var<uniform> total_colors: u32;
@group(2) @binding(2) var base_color_texture: texture_2d<f32>;
@group(2) @binding(3) var base_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let rotation = f32(rotation) / f32(total_colors);
    let color = textureSample(base_color_texture, base_color_sampler, mesh.uv);
    var hsv = rgb2hsv(color.xyz);
    hsv.x += rotation;
    return vec4(hsv2rgb(hsv.x, hsv.y, hsv.z), color.a);
}

fn rgb2hsv(c: vec3f) -> vec3f {
    let K = vec4f(0.0, -0.33333333333333333333, 0.6666666666666666666, -1.0);
    let p = mix(vec4f(c.bg, K.wz), vec4f(c.gb, K.xy), step(c.b, c.g));
    let q = mix(vec4f(p.xyw, c.r), vec4f(c.r, p.yzx), step(p.x, c.r));
    let d = q.x - min(q.w, q.y);
    let e = 1.0e-10;
    return vec3f(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}
