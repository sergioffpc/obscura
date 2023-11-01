use std::sync::Arc;

pub struct PresentPass {
    surface: wgpu::Surface,
    vertex_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

impl PresentPass {
    const QUAD: [f32; 12] = [
        -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
    ];

    pub fn new(
        device: &wgpu::Device,
        surface: wgpu::Surface,
        config: wgpu::SurfaceConfiguration,
        input_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    ) -> Self {
        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&Self::QUAD),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );
        let shader = device.create_shader_module(wgpu::include_wgsl!("present.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("present render pipeline layout"),
            bind_group_layouts: &[&input_bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("present render pipeline"),
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
                cull_mode: None,
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
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            surface,
            vertex_buffer,
            pipeline,
        }
    }

    pub fn pass(
        &self,
        queue: &wgpu::Queue,
        mut encoder: wgpu::CommandEncoder,
        input_bind_group: Arc<wgpu::BindGroup>,
    ) {
        let present_texture = self.surface.get_current_texture().unwrap();
        let present_texture_view = present_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("present render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &present_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &input_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..6, 0..1);
        }
        queue.submit(std::iter::once(encoder.finish()));
        present_texture.present();
    }
}
