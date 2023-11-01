use std::{path::Path, sync::Arc};

use image::{DynamicImage, ImageBuffer, Rgb, Rgba, RgbaImage};
use nalgebra::Matrix4;

use crate::geometry::VertexAttribute;

pub type Scene = petgraph::stable_graph::StableGraph<Node, ()>;

pub fn import<P>(device: &wgpu::Device, queue: &wgpu::Queue, path: P) -> Scene
where
    P: AsRef<Path>,
{
    let (doc, buffers, images) = gltf::import(path).unwrap();
    let textures = doc
        .textures()
        .map(|t| {
            let image = images.get(t.source().index()).unwrap();
            let image_buffer = match image.format {
                gltf::image::Format::R8G8B8 => DynamicImage::ImageRgb8(
                    ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
                        image.width,
                        image.height,
                        image.pixels.clone(),
                    )
                    .unwrap(),
                )
                .to_rgba8(),
                gltf::image::Format::R8G8B8A8 => DynamicImage::ImageRgba8(
                    ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
                        image.width,
                        image.height,
                        image.pixels.clone(),
                    )
                    .unwrap(),
                )
                .to_rgba8(),
                _ => todo!(),
            };

            let mag_filter = t
                .sampler()
                .mag_filter()
                .map_or(Default::default(), |filter| match filter {
                    gltf::texture::MagFilter::Nearest => wgpu::FilterMode::Nearest,
                    gltf::texture::MagFilter::Linear => wgpu::FilterMode::Linear,
                });
            let (min_filter, mipmap_filter) = t.sampler().min_filter().map_or(
                (Default::default(), Default::default()),
                |filter| match filter {
                    gltf::texture::MinFilter::Nearest => {
                        (wgpu::FilterMode::Nearest, Default::default())
                    }
                    gltf::texture::MinFilter::Linear => {
                        (wgpu::FilterMode::Linear, Default::default())
                    }
                    gltf::texture::MinFilter::NearestMipmapNearest => {
                        (wgpu::FilterMode::Nearest, wgpu::FilterMode::Nearest)
                    }
                    gltf::texture::MinFilter::LinearMipmapNearest => {
                        (wgpu::FilterMode::Linear, wgpu::FilterMode::Nearest)
                    }
                    gltf::texture::MinFilter::NearestMipmapLinear => {
                        (wgpu::FilterMode::Nearest, wgpu::FilterMode::Linear)
                    }
                    gltf::texture::MinFilter::LinearMipmapLinear => {
                        (wgpu::FilterMode::Linear, wgpu::FilterMode::Linear)
                    }
                },
            );
            let address_mode_u = match t.sampler().wrap_s() {
                gltf::texture::WrappingMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
                gltf::texture::WrappingMode::MirroredRepeat => wgpu::AddressMode::MirrorRepeat,
                gltf::texture::WrappingMode::Repeat => wgpu::AddressMode::Repeat,
            };
            let address_mode_v = match t.sampler().wrap_t() {
                gltf::texture::WrappingMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
                gltf::texture::WrappingMode::MirroredRepeat => wgpu::AddressMode::MirrorRepeat,
                gltf::texture::WrappingMode::Repeat => wgpu::AddressMode::Repeat,
            };

            Arc::new(
                TextureBuilder::default()
                    .with_address_mode_u(address_mode_u)
                    .with_address_mode_v(address_mode_v)
                    .with_mag_filter(mag_filter)
                    .with_min_filter(min_filter)
                    .with_mipmap_filter(mipmap_filter)
                    .build(device, queue, image_buffer),
            )
        })
        .collect::<Vec<_>>();

    let materials = doc
        .materials()
        .map(|m| {
            let mut builder = MaterialBuilder::default();
            if let Some(tex) = m.emissive_texture() {
                builder.with_emissive(
                    m.emissive_factor(),
                    tex.tex_coord(),
                    textures[tex.texture().index()].clone(),
                );
            }
            if let Some(tex) = m.normal_texture() {
                builder.with_normal(
                    tex.scale(),
                    tex.tex_coord(),
                    textures[tex.texture().index()].clone(),
                );
            }
            if let Some(tex) = m.occlusion_texture() {
                builder.with_occlusion(
                    tex.strength(),
                    tex.tex_coord(),
                    textures[tex.texture().index()].clone(),
                );
            }
            let pbr_metallic_roughness = m.pbr_metallic_roughness();
            if let Some(tex) = pbr_metallic_roughness.base_color_texture() {
                builder.with_base_color(
                    pbr_metallic_roughness.base_color_factor(),
                    tex.tex_coord(),
                    textures[tex.texture().index()].clone(),
                );
            }
            if let Some(tex) = pbr_metallic_roughness.metallic_roughness_texture() {
                builder.with_metallic_roughness(
                    pbr_metallic_roughness.metallic_factor(),
                    pbr_metallic_roughness.roughness_factor(),
                    tex.tex_coord(),
                    textures[tex.texture().index()].clone(),
                );
            }

            Arc::new(builder.build(device, queue))
        })
        .collect::<Vec<_>>();

    let meshes = doc
        .meshes()
        .map(|m| {
            let primitives = m
                .primitives()
                .map(|p| {
                    let reader = p.reader(|buffer| Some(&buffers[buffer.index()]));
                    let mut vertices = reader
                        .read_positions()
                        .unwrap()
                        .map(|position| VertexAttribute {
                            position,
                            ..VertexAttribute::default()
                        })
                        .collect::<Vec<_>>();
                    if let Some(normals) = reader.read_normals() {
                        normals
                            .enumerate()
                            .for_each(|(i, normal)| vertices[i].normal = normal);
                    }
                    if let Some(colors) = reader.read_colors(0) {
                        colors
                            .into_rgba_f32()
                            .enumerate()
                            .for_each(|(i, color_0)| vertices[i].color_0 = color_0);
                    }
                    if let Some(tex_coords) = reader.read_tex_coords(0) {
                        tex_coords
                            .into_f32()
                            .enumerate()
                            .for_each(|(i, tex_coord_0)| vertices[i].tex_coord_0 = tex_coord_0);
                    }
                    let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
                        device,
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(
                                format!("{} vertex buffer", m.name().unwrap_or("unnamed")).as_str(),
                            ),
                            contents: bytemuck::cast_slice(vertices.as_slice()),
                            usage: wgpu::BufferUsages::VERTEX,
                        },
                    );

                    let indices = reader
                        .read_indices()
                        .unwrap()
                        .into_u32()
                        .collect::<Vec<_>>();
                    let index_buffer = wgpu::util::DeviceExt::create_buffer_init(
                        device,
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(
                                format!("{} index buffer", m.name().unwrap_or("unnamed")).as_str(),
                            ),
                            contents: bytemuck::cast_slice(indices.as_slice()),
                            usage: wgpu::BufferUsages::INDEX,
                        },
                    );
                    let material = p.material().index().map_or_else(
                        || Arc::new(MaterialBuilder::default().build(device, queue)),
                        |index| materials[index].clone(),
                    );

                    Arc::new(Primitive {
                        vertex_buffer,
                        index_buffer,
                        index_count: indices.len() as u32,
                        material,
                    })
                })
                .collect::<Vec<_>>();

            Arc::new(Mesh { primitives })
        })
        .collect::<Vec<_>>();

    let mut stable_graph = Scene::new();
    doc.nodes().for_each(|node| {
        let transform_matrix = Matrix4::from(node.transform().matrix());
        let mesh = node.mesh().map(|mesh| meshes[mesh.index()].clone());

        stable_graph.add_node(Node {
            transform_matrix,
            mesh,
        });
    });
    let edges = doc
        .default_scene()
        .unwrap()
        .nodes()
        .flat_map(inorder_traversal_edges)
        .collect::<Vec<_>>();
    stable_graph.extend_with_edges(edges.as_slice());

    stable_graph
}

fn inorder_traversal_edges(node: gltf::Node<'_>) -> Vec<(u32, u32)> {
    node.children()
        .flat_map(|child| {
            std::iter::once((node.index() as u32, child.index() as u32))
                .chain(inorder_traversal_edges(child))
        })
        .collect::<Vec<_>>()
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

#[derive(Default)]
pub struct TextureBuilder {
    address_mode_u: wgpu::AddressMode,
    address_mode_v: wgpu::AddressMode,
    address_mode_w: wgpu::AddressMode,
    mag_filter: wgpu::FilterMode,
    min_filter: wgpu::FilterMode,
    mipmap_filter: wgpu::FilterMode,
}

impl TextureBuilder {
    pub fn with_address_mode_u(mut self, address_mode_u: wgpu::AddressMode) -> Self {
        self.address_mode_u = address_mode_u;
        self
    }

    pub fn with_address_mode_v(mut self, address_mode_v: wgpu::AddressMode) -> Self {
        self.address_mode_v = address_mode_v;
        self
    }

    pub fn with_address_mode_w(mut self, address_mode_w: wgpu::AddressMode) -> Self {
        self.address_mode_w = address_mode_w;
        self
    }

    pub fn with_mag_filter(mut self, mag_filter: wgpu::FilterMode) -> Self {
        self.mag_filter = mag_filter;
        self
    }

    pub fn with_min_filter(mut self, min_filter: wgpu::FilterMode) -> Self {
        self.min_filter = min_filter;
        self
    }

    pub fn with_mipmap_filter(mut self, mipmap_filter: wgpu::FilterMode) -> Self {
        self.mipmap_filter = mipmap_filter;
        self
    }

    pub fn build(self, device: &wgpu::Device, queue: &wgpu::Queue, image: RgbaImage) -> Texture {
        let size = wgpu::Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("RGBA8 texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image.into_raw(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.width),
                rows_per_image: Some(size.height),
            },
            size,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("RGBA8 texture view"),
            ..wgpu::TextureViewDescriptor::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("RGBA8 sampler"),
            address_mode_u: self.address_mode_u,
            address_mode_v: self.address_mode_v,
            mag_filter: self.mag_filter,
            min_filter: self.min_filter,
            mipmap_filter: self.mipmap_filter,
            ..wgpu::SamplerDescriptor::default()
        });

        Texture {
            texture,
            view,
            sampler,
        }
    }

    pub fn from_color(device: &wgpu::Device, queue: &wgpu::Queue, rgba: Rgba<u8>) -> Texture {
        let mut image_buffer = image::RgbaImage::new(1, 1);
        image_buffer.put_pixel(0, 0, rgba);
        Self::default().build(device, queue, image_buffer)
    }
}

pub struct Material {
    pub emissive_factor: [f32; 3],
    pub emissive_tex_coord: u32,
    pub emissive_texture: Arc<Texture>,

    pub normal_scale: f32,
    pub normal_tex_coord: u32,
    pub normal_texture: Arc<Texture>,

    pub occlusion_strength: f32,
    pub occlusion_tex_coord: u32,
    pub occlusion_texture: Arc<Texture>,

    pub base_color_factor: [f32; 4],
    pub base_color_tex_coord: u32,
    pub base_color_texture: Arc<Texture>,

    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub metallic_roughness_tex_coord: u32,
    pub metallic_roughness_texture: Arc<Texture>,

    pub material_bind_group_layout: wgpu::BindGroupLayout,
    pub material_bind_group: wgpu::BindGroup,
}

#[derive(Default)]
pub struct MaterialBuilder {
    emissive_factor: [f32; 3],
    emissive_tex_coord: u32,
    emissive_texture: Option<Arc<Texture>>,

    normal_scale: f32,
    normal_tex_coord: u32,
    normal_texture: Option<Arc<Texture>>,

    occlusion_strength: f32,
    occlusion_tex_coord: u32,
    occlusion_texture: Option<Arc<Texture>>,

    base_color_factor: [f32; 4],
    base_color_tex_coord: u32,
    base_color_texture: Option<Arc<Texture>>,

    metallic_factor: f32,
    roughness_factor: f32,
    metallic_roughness_tex_coord: u32,
    metallic_roughness_texture: Option<Arc<Texture>>,
}

impl MaterialBuilder {
    pub fn with_emissive(
        &mut self,
        emissive_factor: [f32; 3],
        emissive_tex_coord: u32,
        emissive_texture: Arc<Texture>,
    ) -> &Self {
        self.emissive_factor = emissive_factor;
        self.emissive_tex_coord = emissive_tex_coord;
        self.emissive_texture = Some(emissive_texture.clone());
        self
    }

    pub fn with_normal(
        &mut self,
        normal_scale: f32,
        normal_tex_coord: u32,
        normal_texture: Arc<Texture>,
    ) -> &Self {
        self.normal_scale = normal_scale;
        self.normal_tex_coord = normal_tex_coord;
        self.normal_texture = Some(normal_texture.clone());
        self
    }

    pub fn with_occlusion(
        &mut self,
        occlusion_strength: f32,
        occlusion_tex_coord: u32,
        occlusion_texture: Arc<Texture>,
    ) -> &Self {
        self.occlusion_strength = occlusion_strength;
        self.occlusion_tex_coord = occlusion_tex_coord;
        self.occlusion_texture = Some(occlusion_texture.clone());
        self
    }

    pub fn with_base_color(
        &mut self,
        base_color_factor: [f32; 4],
        base_color_tex_coord: u32,
        base_color_texture: Arc<Texture>,
    ) -> &Self {
        self.base_color_factor = base_color_factor;
        self.base_color_tex_coord = base_color_tex_coord;
        self.base_color_texture = Some(base_color_texture.clone());
        self
    }

    pub fn with_metallic_roughness(
        &mut self,
        metallic_factor: f32,
        roughness_factor: f32,
        metallic_roughness_tex_coord: u32,
        metallic_roughness_texture: Arc<Texture>,
    ) -> &Self {
        self.metallic_factor = metallic_factor;
        self.roughness_factor = roughness_factor;
        self.metallic_roughness_tex_coord = metallic_roughness_tex_coord;
        self.metallic_roughness_texture = Some(metallic_roughness_texture.clone());
        self
    }

    pub fn build(self, device: &wgpu::Device, queue: &wgpu::Queue) -> Material {
        let default_texture = Arc::new(TextureBuilder::from_color(
            device,
            queue,
            Rgba([255, 255, 255, 255]),
        ));
        let emissive_texture = match self.emissive_texture {
            Some(tex) => tex.clone(),
            None => default_texture.clone(),
        };
        let normal_texture = match self.normal_texture {
            Some(tex) => tex.clone(),
            None => default_texture.clone(),
        };
        let occlusion_texture = match self.occlusion_texture {
            Some(tex) => tex.clone(),
            None => default_texture.clone(),
        };
        let base_color_texture = match self.base_color_texture {
            Some(tex) => tex.clone(),
            None => default_texture.clone(),
        };
        let metallic_roughness_texture = match self.metallic_roughness_texture {
            Some(tex) => tex.clone(),
            None => default_texture.clone(),
        };
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
                label: Some("material bind group layout"),
                entries: material_bind_group_layout_entries.as_slice(),
            });
        let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("material bind group"),
            layout: &material_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&emissive_texture.as_ref().view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&emissive_texture.as_ref().sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&normal_texture.as_ref().view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&normal_texture.as_ref().sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&occlusion_texture.as_ref().view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&occlusion_texture.as_ref().sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(&base_color_texture.as_ref().view),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: wgpu::BindingResource::Sampler(&base_color_texture.as_ref().sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: wgpu::BindingResource::TextureView(
                        &metallic_roughness_texture.as_ref().view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 9,
                    resource: wgpu::BindingResource::Sampler(
                        &metallic_roughness_texture.as_ref().sampler,
                    ),
                },
            ],
        });

        Material {
            emissive_factor: self.emissive_factor,
            emissive_tex_coord: self.emissive_tex_coord,
            emissive_texture,
            normal_scale: self.normal_scale,
            normal_tex_coord: self.normal_tex_coord,
            normal_texture,
            occlusion_strength: self.occlusion_strength,
            occlusion_tex_coord: self.occlusion_tex_coord,
            occlusion_texture,
            base_color_factor: self.base_color_factor,
            base_color_tex_coord: self.base_color_tex_coord,
            base_color_texture,
            metallic_factor: self.metallic_factor,
            roughness_factor: self.roughness_factor,
            metallic_roughness_tex_coord: self.metallic_roughness_tex_coord,
            metallic_roughness_texture,
            material_bind_group_layout,
            material_bind_group,
        }
    }
}

pub struct Mesh {
    pub primitives: Vec<Arc<Primitive>>,
}

#[derive(Default)]
pub struct Node {
    pub transform_matrix: Matrix4<f32>,
    pub mesh: Option<Arc<Mesh>>,
}

pub struct Primitive {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub material: Arc<Material>,
}
