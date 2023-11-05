use nalgebra::Matrix4;

use crate::renderer::{RenderNode, RenderNodeBuilder};

pub struct PointLight {
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct LightSource {
    position: [f32; 4],
    color: [f32; 4],
}

pub struct LightingPass {
    pub render_node: RenderNode,
    vertex_buffer: wgpu::Buffer,
    lights_bind_group_layout: wgpu::BindGroupLayout,
}

impl LightingPass {
    const LBUFFER_COLOR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;

    const QUAD: [f32; 12] = [
        -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
    ];

    pub fn new(
        device: &wgpu::Device,
        size: wgpu::Extent3d,
        input_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let lights_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("lights"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("quad"),
                contents: bytemuck::cast_slice(&Self::QUAD),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let render_node = RenderNodeBuilder::default()
            .with_name("lighting")
            .with_color_attachment_format(Self::LBUFFER_COLOR_TEXTURE_FORMAT)
            .with_shader_source(include_str!("lighting.wgsl").into())
            .with_bind_group_layout(&input_bind_group_layout)
            .with_bind_group_layout(&lights_bind_group_layout)
            .with_vertex_buffer_layout(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                }],
            })
            .build(&device, size);

        Self {
            render_node,
            vertex_buffer,
            lights_bind_group_layout,
        }
    }

    pub fn pass(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        input_bind_group: &wgpu::BindGroup,
        lights: Vec<(&PointLight, &Matrix4<f32>)>,
    ) {
        let lights_count = lights.len();
        if lights_count == 0 {
            return;
        }

        let lights_count_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("lights count"),
                contents: bytemuck::cast_slice(&[lights_count]),
                usage: wgpu::BufferUsages::UNIFORM,
            },
        );
        let buffer_data = lights
            .iter()
            .map(|(light, transform_matrix)| LightSource {
                position: transform_matrix.column(3).into(),
                color: light.color,
            })
            .collect::<Vec<_>>();
        let lights_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("lights"),
                contents: bytemuck::cast_slice(buffer_data.as_slice()),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            },
        );
        let lights_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("lights"),
            layout: &self.lights_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: lights_count_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: lights_buffer.as_entire_binding(),
                },
            ],
        });

        let mut render_pass = self.render_node.begin_render_pass(encoder);
        render_pass.set_bind_group(0, &input_bind_group, &[]);
        render_pass.set_bind_group(1, &lights_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }
}
