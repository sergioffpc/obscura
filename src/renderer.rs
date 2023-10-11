use legion::{system, world::SubWorld, IntoQuery};
use nalgebra::Matrix4;

use crate::{
    camera::{Projection, View},
    geometry::GeometryPass,
    lighting::{Light, LightingPass},
    present::PresentPass,
    scene::Scene,
};

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
                features: wgpu::Features::empty(),
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
        let geometry_pass = GeometryPass::new(
            &device,
            wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
        );
        let lighting_pass = LightingPass::new(
            &device,
            wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
        );
        let present_pass = PresentPass::new(
            &device,
            surface,
            config,
            geometry_pass.output_bind_group_layout.clone(),
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
#[read_component(Light)]
#[read_component(Matrix4<f32>)]
pub fn present(world: &mut SubWorld, #[resource] renderer: &mut Renderer) {
    let mut encoder = renderer
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("renderer present command encoder"),
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
    let lights = <(&Light, &Matrix4<f32>)>::query()
        .iter(world)
        .collect::<Vec<_>>();
    renderer
        .lighting_pass
        .pass(&renderer.device, &renderer.queue, &mut encoder, lights);
    renderer.present_pass.pass(
        &renderer.queue,
        encoder,
        renderer.geometry_pass.output_bind_group.clone(),
    );
}
