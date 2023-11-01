use std::sync::Arc;

use nalgebra::Matrix4;

pub struct PointLight {
    pub intensity: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct LightSource {
    position: [f32; 4],
    intensity: [f32; 4],
}

pub struct LightingPass {
    pub output_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    pub output_bind_group: Arc<wgpu::BindGroup>,

    lights_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    render_target: wgpu::TextureView,
    vertex_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

impl LightingPass {
    const RENDER_TARGET_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
    const QUAD: [f32; 12] = [
        -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
    ];

    pub fn new(
        device: &wgpu::Device,
        size: wgpu::Extent3d,
        input_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    ) -> Self {
        let light_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("lighting color texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::RENDER_TARGET_TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let light_texture_view = light_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let render_target = light_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let output_bind_group_layout = Arc::new(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("light output bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }],
            },
        ));
        let output_bind_group = Arc::new(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("light output bind group"),
            layout: &output_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&light_texture_view),
            }],
        }));

        let lights_bind_group_layout = Arc::new(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("lighting light bind group layout"),
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
            },
        ));
        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&Self::QUAD),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );
        let shader = device.create_shader_module(wgpu::include_wgsl!("lighting.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("lighting render pipeline layout"),
            bind_group_layouts: &[&input_bind_group_layout, &lights_bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("lighting render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x2,
                    }],
                }],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment",
                targets: &[Some(wgpu::ColorTargetState {
                    format: Self::RENDER_TARGET_TEXTURE_FORMAT,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        LightingPass {
            output_bind_group_layout,
            output_bind_group,
            lights_bind_group_layout,
            render_target,
            vertex_buffer,
            pipeline,
        }
    }

    pub fn pass(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        input_bind_group: Arc<wgpu::BindGroup>,
        lights: Vec<(&PointLight, &Matrix4<f32>)>,
    ) {
        let light_count = lights.len();
        if light_count == 0 {
            return;
        }

        let light_count_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[light_count]),
                usage: wgpu::BufferUsages::UNIFORM,
            },
        );
        let buffer_data = lights
            .iter()
            .map(|(light, transform_matrix)| LightSource {
                position: transform_matrix.column(3).into(),
                intensity: light.intensity,
            })
            .collect::<Vec<_>>();
        let lights_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(buffer_data.as_slice()),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            },
        );
        let lights_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("lighting lights bind group"),
            layout: &self.lights_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: light_count_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: lights_buffer.as_entire_binding(),
                },
            ],
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("lighting render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.render_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &input_bind_group, &[]);
        render_pass.set_bind_group(1, &lights_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }
}
