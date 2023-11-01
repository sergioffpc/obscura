use std::sync::Arc;

use nalgebra::Matrix4;
use petgraph::visit::Dfs;

use crate::scene::Scene;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexAttribute {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color_0: [f32; 4],
    pub tex_coord_0: [f32; 2],
}

impl VertexAttribute {
    const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl Default for VertexAttribute {
    fn default() -> Self {
        Self {
            position: Default::default(),
            normal: Default::default(),
            color_0: [1.0; 4],
            tex_coord_0: Default::default(),
        }
    }
}

pub struct GeometryPass {
    pub output_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    pub output_bind_group: Arc<wgpu::BindGroup>,

    model_matrix_buffer: wgpu::Buffer,
    inv_model_matrix_buffer: wgpu::Buffer,
    view_matrix_buffer: wgpu::Buffer,
    inv_view_matrix_buffer: wgpu::Buffer,
    projection_matrix_buffer: wgpu::Buffer,
    inv_projection_matrix_buffer: wgpu::Buffer,
    transform_bind_group: Arc<wgpu::BindGroup>,
    render_targets: Vec<wgpu::TextureView>,
    depth_stencil_target: wgpu::TextureView,
    pipeline: wgpu::RenderPipeline,
}

impl GeometryPass {
    const RENDER_TARGET_TEXTURE_FORMATS: [wgpu::TextureFormat; 3] = [
        // Position render target texture format..
        wgpu::TextureFormat::Rgba32Float,
        // Normal render target texture format.
        wgpu::TextureFormat::Rgba32Float,
        // Albedo render target texture format.
        wgpu::TextureFormat::Bgra8Unorm,
    ];
    const DEPTH_STENCIL_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;

    pub fn new(device: &wgpu::Device, size: wgpu::Extent3d) -> Self {
        let model_matrix_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("geometry model matrix buffer"),
                contents: bytemuck::cast_slice(Matrix4::<f32>::identity().as_slice()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            },
        );
        let inv_model_matrix_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("geometry model inverse matrix buffer"),
                contents: bytemuck::cast_slice(Matrix4::<f32>::identity().as_slice()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            },
        );
        let view_matrix_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("geometry view matrix buffer"),
                contents: bytemuck::cast_slice(Matrix4::<f32>::identity().as_slice()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            },
        );
        let inv_view_matrix_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("geometry view inverse matrix buffer"),
                contents: bytemuck::cast_slice(Matrix4::<f32>::identity().as_slice()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            },
        );
        let projection_matrix_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("geometry projection matrix buffer"),
                contents: bytemuck::cast_slice(Matrix4::<f32>::identity().as_slice()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            },
        );
        let inv_projection_matrix_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("geometry projection inverse matrix buffer"),
                contents: bytemuck::cast_slice(Matrix4::<f32>::identity().as_slice()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            },
        );
        let transform_bind_group_layout_entries = (0..6)
            .map(|index| wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect::<Vec<_>>();
        let transform_bind_group_layout = Arc::new(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("geometry transform bind group layout"),
                entries: transform_bind_group_layout_entries.as_slice(),
            },
        ));
        let transform_bind_group = Arc::new(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("geometry transform bind group"),
            layout: &transform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_matrix_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: inv_model_matrix_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: view_matrix_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: inv_view_matrix_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: projection_matrix_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: inv_projection_matrix_buffer.as_entire_binding(),
                },
            ],
        }));
        let render_targets = Self::RENDER_TARGET_TEXTURE_FORMATS
            .iter()
            .map(|&format| {
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                });
                texture.create_view(&wgpu::TextureViewDescriptor::default())
            })
            .collect::<Vec<_>>();
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("geometry depth texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: GeometryPass::DEPTH_STENCIL_TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let depth_stencil_target =
            depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let output_bind_group_layout_entries = (0..render_targets.len())
            .map(|index| wgpu::BindGroupLayoutEntry {
                binding: index as u32,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            })
            .chain(std::iter::once(wgpu::BindGroupLayoutEntry {
                binding: render_targets.len() as u32,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Depth,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }))
            .collect::<Vec<_>>();
        let output_bind_group_layout = Arc::new(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("geometry output bind group layout"),
                entries: output_bind_group_layout_entries.as_slice(),
            },
        ));
        let output_bind_group_entries = render_targets
            .iter()
            .chain(std::iter::once(&depth_stencil_target))
            .enumerate()
            .map(|(i, texture_view)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: wgpu::BindingResource::TextureView(texture_view),
            })
            .collect::<Vec<_>>();
        let output_bind_group = Arc::new(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("geometry output bind group"),
            layout: &output_bind_group_layout,
            entries: output_bind_group_entries.as_slice(),
        }));
        let material_bind_group_layout_entries = (0..5)
            .flat_map(|i| {
                [
                    wgpu::BindGroupLayoutEntry {
                        binding: (i * 2) as u32,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: (i * 2 + 1) as u32,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ]
            })
            .collect::<Vec<_>>();
        let material_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("geometry bind group layout"),
                entries: material_bind_group_layout_entries.as_slice(),
            });
        let shader = device.create_shader_module(wgpu::include_wgsl!("geometry.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("geometry render pipeline layout"),
            bind_group_layouts: &[&transform_bind_group_layout, &material_bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("geometry render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex",
                buffers: &[VertexAttribute::desc()],
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: GeometryPass::DEPTH_STENCIL_TEXTURE_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment",
                targets: Self::RENDER_TARGET_TEXTURE_FORMATS
                    .map(|format| {
                        Some(wgpu::ColorTargetState {
                            format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })
                    })
                    .as_slice(),
            }),
            multiview: None,
        });

        Self {
            output_bind_group_layout,
            output_bind_group,
            model_matrix_buffer,
            inv_model_matrix_buffer,
            view_matrix_buffer,
            inv_view_matrix_buffer,
            projection_matrix_buffer,
            inv_projection_matrix_buffer,
            transform_bind_group,
            render_targets,
            depth_stencil_target,
            pipeline,
        }
    }

    pub fn pass(
        &self,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        projection_matrix: Matrix4<f32>,
        view_matrix: Matrix4<f32>,
        geometries: Vec<(&Scene, &Matrix4<f32>)>,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("geometry render pass"),
            color_attachments: self
                .render_targets
                .iter()
                .map(|target| {
                    Some(wgpu::RenderPassColorAttachment {
                        view: target,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                            store: true,
                        },
                    })
                })
                .collect::<Vec<_>>()
                .as_slice(),
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_stencil_target,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });
        render_pass.set_pipeline(&self.pipeline);
        queue.write_buffer(
            &self.projection_matrix_buffer,
            0,
            bytemuck::cast_slice(projection_matrix.as_slice()),
        );
        let inv_projection_matrix = projection_matrix.try_inverse().unwrap();
        queue.write_buffer(
            &self.inv_projection_matrix_buffer,
            0,
            bytemuck::cast_slice(inv_projection_matrix.as_slice()),
        );
        queue.write_buffer(
            &self.view_matrix_buffer,
            0,
            bytemuck::cast_slice(view_matrix.as_slice()),
        );
        let inv_view_matrix = view_matrix.try_inverse().unwrap();
        queue.write_buffer(
            &self.inv_view_matrix_buffer,
            0,
            bytemuck::cast_slice(inv_view_matrix.as_slice()),
        );
        render_pass.set_bind_group(0, &self.transform_bind_group, &[]);
        geometries.iter().for_each(|(scene, transform_matrix)| {
            let mut dfs = Dfs::new(scene, petgraph::graph::node_index(0));
            while let Some(index) = dfs.next(scene) {
                let node = &scene[index];
                if let Some(mesh) = &node.mesh {
                    let model_matrix = node.transform_matrix
                        * dfs.stack.iter().fold(Matrix4::identity(), |acc, &nx| {
                            acc * scene[nx].transform_matrix
                        })
                        * *transform_matrix;
                    queue.write_buffer(
                        &self.model_matrix_buffer,
                        0,
                        bytemuck::cast_slice(model_matrix.as_slice()),
                    );
                    let inv_model_matrix = model_matrix.try_inverse().unwrap();
                    queue.write_buffer(
                        &self.inv_model_matrix_buffer,
                        0,
                        bytemuck::cast_slice(inv_model_matrix.as_slice()),
                    );
                    mesh.primitives.iter().for_each(|primitive| {
                        render_pass.set_bind_group(1, &primitive.material.material_bind_group, &[]);
                        render_pass.set_vertex_buffer(0, primitive.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(
                            primitive.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint32,
                        );
                        render_pass.draw_indexed(0..primitive.index_count, 0, 0..1);
                    });
                }
            }
        });
    }
}
