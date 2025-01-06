use anyhow::Result;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<()> {
    let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_resizable(false)
        .with_title("Gpu PT".to_string())
        .build(&event_loop)?;

    event_loop.run(|event, control_handle| {
        control_handle.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_handle.exit(),
                WindowEvent::RedrawRequested => {
                    window.request_redraw();
                }
                _ => (),
            },
            _ => (),
        }
    })?;

    Ok(())
}
