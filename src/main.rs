#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[macro_use]
extern crate log;

use std::{
    thread,
    time::{Duration, Instant},
};

use clap::Parser;
use legion::{system, world::SubWorld, IntoQuery, Resources, Schedule, World};
use nalgebra::{Matrix4, Vector3};
use obscura::{
    camera::{Projection, View},
    renderer::{present_system, Renderer},
    scene,
};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value_t = 1280)]
    width: u32,

    #[arg(long, default_value_t = 720)]
    height: u32,

    #[arg(long, default_value_t = 120)]
    frame_count: u32,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    let target_frame_time = Duration::from_secs_f64(1.0 / f64::from(cli.frame_count));
    let mut entity_world = World::default();
    let mut shared_resources = Resources::default();
    let mut entity_scheduler = Schedule::builder().add_system(input_system()).build();
    let mut render_scheduler = Schedule::builder().add_system(present_system()).build();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(cli.width, cli.height))
        .with_resizable(false)
        .with_title(format!(
            "{} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .build(&event_loop)
        .unwrap();
    let renderer = Renderer::new(&window);
    let camera = Projection::new(window.inner_size().width, window.inner_size().height);
    entity_world.push((camera, View::default()));
    let scene = scene::import(&renderer.device, &renderer.queue, "res/BoxVertexColors.glb");
    entity_world.push((scene, Matrix4::<f32>::identity()));
    shared_resources.insert(renderer);
    let mut keyboard_keycode: Option<VirtualKeyCode> = None;
    let mut mouse_motion: Option<(f64, f64)> = None;
    let mut last_update_time = Instant::now();
    let mut last_frame_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.set_exit(),
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => match state {
                    ElementState::Pressed => keyboard_keycode = virtual_keycode,
                    ElementState::Released => keyboard_keycode = None,
                },
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                winit::event::DeviceEvent::MouseMotion { delta } => mouse_motion = Some(delta),
                _ => (),
            },
            Event::MainEventsCleared => {
                shared_resources.insert(keyboard_keycode);
                shared_resources.insert(mouse_motion);
                shared_resources.insert(last_update_time.elapsed());
                entity_scheduler.execute(&mut entity_world, &mut shared_resources);
                last_update_time = Instant::now();
                mouse_motion = None;

                render_scheduler.execute(&mut entity_world, &mut shared_resources);
            }
            _ => (),
        }
        let elapsed_frame_time = last_frame_time.elapsed();
        if elapsed_frame_time < target_frame_time {
            let sleep_duration = target_frame_time - elapsed_frame_time;
            thread::sleep(sleep_duration);
        }
        last_frame_time = Instant::now();
    });
}

#[system]
#[read_component(Projection)]
#[write_component(View)]
pub fn input(
    world: &mut SubWorld,
    #[resource] keyboard_keycode: &Option<VirtualKeyCode>,
    #[resource] mouse_motion: &Option<(f64, f64)>,
    #[resource] delta_time: &Duration,
) {
    let (_, view) = <(&Projection, &mut View)>::query()
        .iter_mut(world)
        .next()
        .unwrap();
    let step_factor = 1.0 * delta_time.as_secs_f32();
    if let Some(keyboard_keycode) = keyboard_keycode {
        if *keyboard_keycode == VirtualKeyCode::Home {
            *view = View::default();
        } else {
            let step = match *keyboard_keycode {
                VirtualKeyCode::Up => -Vector3::z(),
                VirtualKeyCode::Down => Vector3::z(),
                VirtualKeyCode::PageDown => -Vector3::y(),
                VirtualKeyCode::PageUp => Vector3::y(),
                VirtualKeyCode::Left => -Vector3::x(),
                VirtualKeyCode::Right => Vector3::x(),
                _ => Vector3::zeros(),
            } * step_factor;
            view.position += step;
        }
    }
    if let Some((x, y)) = mouse_motion {
        view.yaw += *x as f32 * step_factor;
        view.pitch += *y as f32 * step_factor;
    }
}
