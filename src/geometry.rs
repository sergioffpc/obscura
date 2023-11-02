use std::sync::Arc;

use nalgebra::Matrix4;
use petgraph::visit::Dfs;

use crate::{
    renderer::{RenderNode, RenderNodeBuilder},
    scene::Scene,
};

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
    pub render_node: RenderNode,
    transform_buffers: Vec<wgpu::Buffer>,
    transform_bind_group: wgpu::BindGroup,
}

impl GeometryPass {
    const GBUFFER_POSITION_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
    const GBUFFER_NORMAL_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
    const GBUFFER_ALBEDO_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
    const GBUFFER_DEPTH_STENCIL_TEXTURE_FORMAT: wgpu::TextureFormat =
        wgpu::TextureFormat::Depth24Plus;

    const TRANSFORM_MODEL_MATRIX_IDX: usize = 0;
    const TRANSFORM_MODEL_INV_MATRIX_IDX: usize = 1;
    const TRANSFORM_VIEW_MATRIX_IDX: usize = 2;
    const TRANSFORM_VIEW_INV_MATRIX_IDX: usize = 3;
    const TRANSFORM_PROJECTION_MATRIX_IDX: usize = 4;
    const TRANSFORM_PROJECTION_INV_MATRIX_IDX: usize = 5;

    pub fn new(device: &wgpu::Device, size: wgpu::Extent3d) -> Self {
        let mut transform_buffers = vec![];
        (0..6).for_each(|_| {
            let buffer = wgpu::util::DeviceExt::create_buffer_init(
                device,
                &wgpu::util::BufferInitDescriptor {
                    label: Some("transform"),
                    contents: bytemuck::cast_slice(Matrix4::<f32>::identity().as_slice()),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                },
            );
            transform_buffers.push(buffer);
        });

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
                label: Some("transform"),
                entries: transform_bind_group_layout_entries.as_slice(),
            },
        ));

        let transform_bind_group_entries = (0..6)
            .map(|index| wgpu::BindGroupEntry {
                binding: index,
                resource: transform_buffers[index as usize].as_entire_binding(),
            })
            .collect::<Vec<_>>();
        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("transform"),
            layout: &transform_bind_group_layout,
            entries: transform_bind_group_entries.as_slice(),
        });

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
                label: Some("material"),
                entries: material_bind_group_layout_entries.as_slice(),
            });

        let render_node = RenderNodeBuilder::default()
            .with_name("geometry")
            .with_color_attachment_format(Self::GBUFFER_POSITION_TEXTURE_FORMAT)
            .with_color_attachment_format(Self::GBUFFER_NORMAL_TEXTURE_FORMAT)
            .with_color_attachment_format(Self::GBUFFER_ALBEDO_TEXTURE_FORMAT)
            .with_depth_stencil_format(Self::GBUFFER_DEPTH_STENCIL_TEXTURE_FORMAT)
            .with_shader_source(include_str!("geometry.wgsl").into())
            .with_bind_group_layout(&transform_bind_group_layout)
            .with_bind_group_layout(&material_bind_group_layout)
            .with_vertex_buffer_layout(VertexAttribute::desc())
            .build(&device, size);

        Self {
            render_node,
            transform_buffers,
            transform_bind_group,
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
        let mut render_pass = self.render_node.begin_render_pass(encoder);

        [
            (Self::TRANSFORM_PROJECTION_MATRIX_IDX, projection_matrix),
            (
                Self::TRANSFORM_PROJECTION_INV_MATRIX_IDX,
                projection_matrix.try_inverse().unwrap(),
            ),
            (Self::TRANSFORM_VIEW_MATRIX_IDX, view_matrix),
            (
                Self::TRANSFORM_VIEW_INV_MATRIX_IDX,
                view_matrix.try_inverse().unwrap(),
            ),
        ]
        .iter()
        .for_each(|(index, matrix)| {
            queue.write_buffer(
                &self.transform_buffers[*index],
                0,
                bytemuck::cast_slice(matrix.as_slice()),
            )
        });

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
                        &self.transform_buffers[Self::TRANSFORM_MODEL_MATRIX_IDX],
                        0,
                        bytemuck::cast_slice(model_matrix.as_slice()),
                    );
                    let inv_model_matrix = model_matrix.try_inverse().unwrap();
                    queue.write_buffer(
                        &self.transform_buffers[Self::TRANSFORM_MODEL_INV_MATRIX_IDX],
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
