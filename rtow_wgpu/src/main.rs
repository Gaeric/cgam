use anyhow::Result;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{self, EventLoop};
use winit::window::Window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[derive(Default)]
struct Application {
    widnow: Option<Window>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
        let window_attributes = Window::default_attributes()
            .with_title("Gpu PT".to_string())
            .with_inner_size(window_size)
            .with_resizable(false);
        self.widnow = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        println!("event is {event:?}");

        let window = match self.widnow.as_ref() {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => window.request_redraw(),
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    println!("hello world");

    let mut app = Application::default();

    let _ = event_loop.run_app(&mut app);

    Ok(())
}
