use winit::{
    event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

mod state;
mod resources;
mod texture;

pub use state::State;


pub fn hello() {
    println!("Hello, world!");
}

pub async fn run<T: state::State>(state: &mut T, event_loop: EventLoop<()>) {
    env_logger::init();
    // let mut last_render_time = instant::Instant::now();

    let _ = event_loop
        .run(move |event, control_flow| match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                log::debug!("delta is {delta:?}");
                // handle mouse event
            }
            Event::WindowEvent { window_id, event } if window_id == state.window().id() => {
                if !state.input(&event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            log::info!("physical size: {physical_size:?}");
                            state.resize(physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    let size = state.size();
                                    state.resize(size);
                                }
                                Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                                Err(wgpu::SurfaceError::Timeout) => log::warn!("surface timeout"),
                            }
                        }
                        _ => {}
                    }
                } else {
                    state.window().request_redraw();
                }
            }
            _ => {
                log::info!("event run");
            }
        })
        .unwrap();
}
