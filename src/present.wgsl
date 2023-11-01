@vertex fn vertex(@location(0) in_position : vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(in_position, 0.0, 1.0);
}

@group(0) @binding(0) var l_buffer_color : texture_2d<f32>;

@fragment fn fragment(@builtin(position) in_position : vec4<f32>) -> @location(0) vec4<f32> {
    let color = textureLoad(l_buffer_color, vec2<i32>(floor(in_position.xy)), 0).rgb;

    return vec4<f32>(color, 1.0);
}
