@vertex fn vertex(@location(0) in_position : vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(in_position, 0.0, 1.0);
}

@group(0) @binding(0) var<uniform> light_count : u32;
struct light_storage {
    position : vec3<f32>,
    color    : vec3<f32>,
}
@group(0) @binding(1) var<storage, read> lights: array<light_storage>;

@fragment fn fragment(@builtin(position) in_position : vec4<f32>) -> @location(0) vec4<f32> {
    for (var i = 0u; i < light_count; i++) {
        
    }
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}
