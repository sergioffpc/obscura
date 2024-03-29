@group(0) @binding(0) var<uniform> model_matrix          : mat4x4<f32>;
@group(0) @binding(1) var<uniform> inv_model_matrix      : mat4x4<f32>;
@group(0) @binding(2) var<uniform> view_matrix           : mat4x4<f32>;
@group(0) @binding(3) var<uniform> inv_view_matrix       : mat4x4<f32>;
@group(0) @binding(4) var<uniform> projection_matrix     : mat4x4<f32>;
@group(0) @binding(5) var<uniform> inv_projection_matrix : mat4x4<f32>;

struct VertexOutput {
    @builtin(position) position : vec4<f32>,
    @location(0) normal         : vec3<f32>,
    @location(1) color_0        : vec4<f32>,
    @location(2) tex_coord_0    : vec2<f32>,
}

@vertex fn vertex(
    @location(0) in_position    : vec3<f32>,
    @location(1) in_normal      : vec3<f32>,
    @location(2) in_color_0     : vec4<f32>,
    @location(3) in_tex_coord_0 : vec2<f32>,
) -> VertexOutput {
    let world_position = model_matrix * vec4(in_position, 1.0);
    let view_position = view_matrix * world_position;
    let clip_position = projection_matrix * view_position;

    var out : VertexOutput;
    out.position = clip_position;
    out.normal = normalize((transpose(inv_model_matrix) * vec4<f32>(in_normal, 0.0)).xyz);
    out.color_0 = in_color_0;
    out.tex_coord_0 = vec2<f32>(in_tex_coord_0.x, 1.0 - in_tex_coord_0.y);

    return out;
}

@group(1) @binding(0) var emissive_texture           : texture_2d<f32>;
@group(1) @binding(1) var emissive_sampler           : sampler;
@group(1) @binding(2) var normal_texture             : texture_2d<f32>;
@group(1) @binding(3) var normal_sampler             : sampler;
@group(1) @binding(4) var occlusion_texture          : texture_2d<f32>;
@group(1) @binding(5) var occlusion_sampler          : sampler;
@group(1) @binding(6) var base_color_texture         : texture_2d<f32>;
@group(1) @binding(7) var base_color_sampler         : sampler;
@group(1) @binding(8) var metallic_roughness_texture : texture_2d<f32>;
@group(1) @binding(9) var metallic_roughness_sampler : sampler;

struct FragmentOutput {
    @location(0) position : vec4<f32>,
    @location(1) normal   : vec4<f32>,
    @location(2) albedo   : vec4<f32>,
}

@fragment fn fragment(
    @builtin(position) in_position : vec4<f32>,
    @location(0) in_normal         : vec3<f32>,
    @location(1) in_color_0        : vec4<f32>,
    @location(2) in_tex_coord_0    : vec2<f32>,
) -> FragmentOutput {
    var out : FragmentOutput;
    out.position = in_position;
    out.normal = vec4(in_normal, 1.0);

    let emissive_color = textureSample(emissive_texture, emissive_sampler, in_tex_coord_0);
    let normal_color = textureSample(normal_texture, normal_sampler, in_tex_coord_0);
    let occlusion_color = textureSample(occlusion_texture, occlusion_sampler, in_tex_coord_0);
    let base_color_color = textureSample(base_color_texture, base_color_sampler, in_tex_coord_0);
    let metallic_roughness_color = textureSample(metallic_roughness_texture, metallic_roughness_sampler, in_tex_coord_0);
    out.albedo = in_color_0 * emissive_color * normal_color * occlusion_color * base_color_color * metallic_roughness_color;
    
    return out;
}
