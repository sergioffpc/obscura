use crate::renderer::{RenderNode, RenderNodeBuilder};

pub struct PresentPass {
    render_node: RenderNode,
    surface: wgpu::Surface,
    vertex_buffer: wgpu::Buffer,
}

impl PresentPass {
    const QUAD: [f32; 12] = [
        -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
    ];

    pub fn new(
        device: &wgpu::Device,
        surface: wgpu::Surface,
        config: wgpu::SurfaceConfiguration,
        input_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("quad"),
                contents: bytemuck::cast_slice(&Self::QUAD),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let render_node = RenderNodeBuilder::default()
            .with_name("present")
            .with_color_attachment_format(config.format)
            .with_shader_source(include_str!("present.wgsl").into())
            .with_bind_group_layout(&input_bind_group_layout)
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
            surface,
            vertex_buffer,
        }
    }

    pub fn pass(
        &self,
        queue: &wgpu::Queue,
        mut encoder: wgpu::CommandEncoder,
        input_bind_group: &wgpu::BindGroup,
    ) {
        let present_texture = self.surface.get_current_texture().unwrap();
        let present_texture_view = present_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
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
            render_pass.set_pipeline(&self.render_node.render_pipeline);
            render_pass.set_bind_group(0, &input_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..6, 0..1);
        }
        queue.submit(std::iter::once(encoder.finish()));
        present_texture.present();
    }
}
