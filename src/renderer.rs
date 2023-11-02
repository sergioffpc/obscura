use std::borrow::Cow;

use legion::{system, world::SubWorld, IntoQuery};
use nalgebra::Matrix4;

use crate::{
    camera::{Projection, View},
    geometry::GeometryPass,
    lighting::{LightingPass, PointLight},
    present::PresentPass,
    scene::Scene,
};

pub struct RenderTarget {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl RenderTarget {
    pub fn new(
        device: &wgpu::Device,
        label: wgpu::Label,
        color_attachments: &[wgpu::TextureView],
        depth_stencil_attachment: &Option<wgpu::TextureView>,
    ) -> Self {
        let mut bind_group_layout_entries = (0..color_attachments.len())
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
            .collect::<Vec<_>>();

        if depth_stencil_attachment.is_some() {
            bind_group_layout_entries.push(wgpu::BindGroupLayoutEntry {
                binding: color_attachments.len() as u32,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Depth,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            });
        }
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: bind_group_layout_entries.as_slice(),
        });

        let mut bind_group_entries = color_attachments
            .iter()
            .enumerate()
            .map(|(i, texture_view)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: wgpu::BindingResource::TextureView(texture_view),
            })
            .collect::<Vec<_>>();

        if let Some(texture_view) = depth_stencil_attachment {
            bind_group_entries.push(wgpu::BindGroupEntry {
                binding: bind_group_entries.len() as u32,
                resource: wgpu::BindingResource::TextureView(texture_view),
            });
        }
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &bind_group_layout,
            entries: bind_group_entries.as_slice(),
        });

        Self {
            bind_group_layout,
            bind_group,
        }
    }
}

#[derive(Default)]
pub struct RenderNodeBuilder<'a> {
    label: wgpu::Label<'a>,
    color_attachment_formats: Vec<wgpu::TextureFormat>,
    depth_stencil_format: Option<wgpu::TextureFormat>,
    shader_source: Cow<'a, str>,
    bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,
    vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'a>>,
}

impl<'a> RenderNodeBuilder<'a> {
    pub fn with_name(mut self, name: &'static str) -> Self {
        self.label = Some(name);
        self
    }

    pub fn with_color_attachment_format(mut self, format: wgpu::TextureFormat) -> Self {
        self.color_attachment_formats.push(format);
        self
    }

    pub fn with_depth_stencil_format(mut self, format: wgpu::TextureFormat) -> Self {
        self.depth_stencil_format = Some(format);
        self
    }

    pub fn with_shader_source(mut self, source: Cow<'a, str>) -> Self {
        self.shader_source = source;
        self
    }

    pub fn with_bind_group_layout(mut self, layout: &'a wgpu::BindGroupLayout) -> Self {
        self.bind_group_layouts.push(layout);
        self
    }

    pub fn with_vertex_buffer_layout(mut self, layout: wgpu::VertexBufferLayout<'a>) -> Self {
        self.vertex_buffer_layouts.push(layout);
        self
    }

    pub fn build(self, device: &wgpu::Device, size: wgpu::Extent3d) -> RenderNode {
        let color_views = self
            .color_attachment_formats
            .iter()
            .map(|&format| {
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: self.label,
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
        let depth_stencil_view = self.depth_stencil_format.map(|format| {
            let depth_stencil_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: self.label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            depth_stencil_texture.create_view(&wgpu::TextureViewDescriptor::default())
        });
        let render_target = RenderTarget::new(
            device,
            self.label,
            color_views.as_slice(),
            &depth_stencil_view,
        );
        let render_pipeline = self.create_render_pipeline(device);

        RenderNode {
            render_target,
            color_views,
            depth_stencil_view,
            render_pipeline,
        }
    }

    fn create_render_pipeline(self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        let depth_stencil = self
            .depth_stencil_format
            .map(|format| wgpu::DepthStencilState {
                format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: self.label,
            source: wgpu::ShaderSource::Wgsl(self.shader_source),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: self.label,
                bind_group_layouts: self.bind_group_layouts.as_slice(),
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: self.label,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex",
                buffers: self.vertex_buffer_layouts.as_slice(),
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
            depth_stencil,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment",
                targets: self
                    .color_attachment_formats
                    .iter()
                    .map(|&format| {
                        Some(wgpu::ColorTargetState {
                            format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
            }),
            multiview: None,
        })
    }
}

pub struct RenderNode {
    pub render_target: RenderTarget,
    pub render_pipeline: wgpu::RenderPipeline,
    color_views: Vec<wgpu::TextureView>,
    depth_stencil_view: Option<wgpu::TextureView>,
}

impl RenderNode {
    pub fn begin_render_pass<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
    ) -> wgpu::RenderPass {
        let color_attachments = self
            .color_views
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
            .collect::<Vec<_>>();
        let depth_stencil_attachment =
            self.depth_stencil_view
                .as_ref()
                .map(|view| wgpu::RenderPassDepthStencilAttachment {
                    view: &view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                });

        let render_pass_desc = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: color_attachments.as_slice(),
            depth_stencil_attachment,
        };

        let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass
    }
}

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    geometry_pass: GeometryPass,
    lighting_pass: LightingPass,
    present_pass: PresentPass,
}

impl Renderer {
    pub fn new(window: &winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                limits: wgpu::Limits::default(),
            },
            None,
        ))
        .unwrap();
        let format = surface
            .get_capabilities(&adapter)
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let geometry_pass = GeometryPass::new(&device, size);
        let lighting_pass = LightingPass::new(
            &device,
            size,
            &geometry_pass.render_node.render_target.bind_group_layout,
        );
        let present_pass = PresentPass::new(
            &device,
            surface,
            config,
            &lighting_pass.render_node.render_target.bind_group_layout,
        );

        Self {
            device,
            queue,
            geometry_pass,
            lighting_pass,
            present_pass,
        }
    }
}

#[system]
#[read_component(Projection)]
#[read_component(View)]
#[read_component(Scene)]
#[read_component(PointLight)]
#[read_component(Matrix4<f32>)]
pub fn present(world: &mut SubWorld, #[resource] renderer: &mut Renderer) {
    let mut encoder = renderer
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("present"),
        });
    let (projection, view) = <(&Projection, &View)>::query().iter(world).next().unwrap();

    let geometries = <(&Scene, &Matrix4<f32>)>::query()
        .iter(world)
        .collect::<Vec<_>>();
    renderer.geometry_pass.pass(
        &renderer.queue,
        &mut encoder,
        projection.as_matrix(),
        view.as_matrix(),
        geometries,
    );

    let lights = <(&PointLight, &Matrix4<f32>)>::query()
        .iter(world)
        .collect::<Vec<_>>();
    renderer.lighting_pass.pass(
        &renderer.device,
        &mut encoder,
        &renderer.geometry_pass.render_node.render_target.bind_group,
        lights,
    );

    renderer.present_pass.pass(
        &renderer.queue,
        encoder,
        &renderer.lighting_pass.render_node.render_target.bind_group,
    );
}
