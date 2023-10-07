@group(0) @binding(0) var<uniform> model_view_projection : mat4x4<f32>;

struct vertex_output {
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
) -> vertex_output {
  var output : vertex_output;
  output.position = model_view_projection * vec4(in_position, 1.0);
  output.normal = in_normal; // convert to world coords
  output.color_0 = in_color_0;
  output.tex_coord_0 = vec2<f32>(in_tex_coord_0.x, 1.0 - in_tex_coord_0.y);

  return output;
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

struct fragment_output {
  @location(0) position : vec4<f32>,
  @location(1) normal   : vec4<f32>,
  @location(2) albedo   : vec4<f32>,
}

@fragment fn fragment(
  @builtin(position) in_position : vec4<f32>,
  @location(0) in_normal         : vec3<f32>,
  @location(1) in_color_0        : vec4<f32>,
  @location(2) in_tex_coord_0    : vec2<f32>,
) -> fragment_output {
  var output : fragment_output;
  output.position = in_position;
  output.normal = vec4(in_normal, 1.0);

  let emissive_color = textureSample(emissive_texture, emissive_sampler, in_tex_coord_0);
  let normal_color = textureSample(normal_texture, normal_sampler, in_tex_coord_0);
  let occlusion_color = textureSample(occlusion_texture, occlusion_sampler, in_tex_coord_0);
  let base_color_color = textureSample(base_color_texture, base_color_sampler, in_tex_coord_0);
  let metallic_roughness_color = textureSample(metallic_roughness_texture, metallic_roughness_sampler, in_tex_coord_0);
  output.albedo = in_color_0 * emissive_color * normal_color * occlusion_color * base_color_color * metallic_roughness_color;

  return output;
}
