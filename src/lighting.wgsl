@vertex fn vertex(@location(0) in_position : vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(in_position, 0.0, 1.0);
}

@group(0) @binding(0) var g_buffer_position : texture_2d<f32>;
@group(0) @binding(1) var g_buffer_normal   : texture_2d<f32>;
@group(0) @binding(2) var g_buffer_albedo   : texture_2d<f32>;
@group(0) @binding(3) var g_buffer_depth    : texture_depth_2d;

@group(1) @binding(0) var<uniform> light_count : u32;
struct LightSource {
    position : vec4<f32>,
    color    : vec4<f32>,
}
@group(1) @binding(1) var<storage, read> lights: array<LightSource>;

@fragment fn fragment(@builtin(position) in_position : vec4<f32>) -> @location(0) vec4<f32> {
    let position = textureLoad(g_buffer_position, vec2<i32>(floor(in_position.xy)), 0).xyz;
    let normal = textureLoad(g_buffer_normal, vec2<i32>(floor(in_position.xy)), 0).xyz;
    let albedo = textureLoad(g_buffer_albedo, vec2<i32>(floor(in_position.xy)), 0).rgb;
    let depth = textureLoad(g_buffer_depth, vec2<i32>(floor(in_position.xy)), 0);

    if depth >= 1.0 {
        return vec4<f32>(albedo, 0.0);
    }

    let N = normalize(normal);

    var surface_color = vec3<f32>(0.0, 0.0, 0.0);
    for (var i = 0u; i < light_count; i++) {
        let world_to_light = lights[i].position.xyz - position;
        let dist = length(world_to_light);
        let wi = normalize(world_to_light);

        let radiance = lights[i].color.rgb * (1.0 / pow(dist, 2.0));
        let n_dot_l = max(dot(N, wi), 0.0);

        surface_color += albedo * radiance * n_dot_l;
    }
    return vec4<f32>(surface_color, 1.0);
}
