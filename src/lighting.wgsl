@vertex fn vertex(@location(0) in_position : vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(in_position, 0.0, 1.0);
}

@group(0) @binding(0) var g_buffer_position : texture_2d<f32>;
@group(0) @binding(1) var g_buffer_normal   : texture_2d<f32>;
@group(0) @binding(2) var g_buffer_albedo   : texture_2d<f32>;
@group(0) @binding(3) var g_buffer_depth    : texture_depth_2d;

@group(1) @binding(0) var<uniform> light_count : u32;
struct LightSource {
    position  : vec4<f32>,
    intensity : vec4<f32>,
}
@group(1) @binding(1) var<storage, read> lights: array<LightSource>;

struct FragmentOutput {
    @location(0) color : vec4<f32>,
}

@fragment fn fragment(@builtin(position) in_position : vec4<f32>) -> FragmentOutput {
    let position = textureLoad(g_buffer_position, vec2<i32>(floor(in_position.xy)), 0).xyz;
    let normal = textureLoad(g_buffer_normal, vec2<i32>(floor(in_position.xy)), 0).xyz;
    let albedo = textureLoad(g_buffer_albedo, vec2<i32>(floor(in_position.xy)), 0).rgb;
    let depth = textureLoad(g_buffer_depth, vec2<i32>(floor(in_position.xy)), 0);

    var out : FragmentOutput;
    if depth < 1.0 {
        out.color = vec4(albedo, 1.0) * 0.01;
        for (var i = 0u; i < light_count; i++) {
            out.color += lights[i].intensity;
        }
    }
    return out;
}
