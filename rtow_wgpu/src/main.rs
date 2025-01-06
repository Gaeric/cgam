use anyhow::{Context, Result};
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

async fn connect_to_gpu(window: &Window) -> Result<(wgpu::Device, wgpu::Queue, wgpu::Surface)> {
    // Create an "instance" of wgpu. This is the entry-point to the API.
    let instance = wgpu::Instance::default();

    // Create a drawable "surface" that is associated with the window.
    let surface = instance.create_surface(window)?;

    // Request a GPU that is compatible with the surface. If the system has multiple GPUs then
    // pick the hight performance one.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .context("failed to faind a compatible adapter")?;

    // Connect to the GPU. "device" represents the connection to the GPU and allows us to create
    // resources like buffers, textures, and pipelines. "queue" represents the command queue that
    // we use to submit commands to the GPU.
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .context("failed to connect to the GPU")?;

    // Configure the texture memory backs the surface. Our renderer will draw to a surface texture
    // every frame.
    let caps = surface.get_capabilities(&adapter);
    let format = caps
        .formats
        .into_iter()
        .find(|it| {
            matches!(
                it,
                wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Bgra8Unorm
            )
        })
        .context("could not find preferred texture format (Rgba8Unorm or Bgra8Unorm)")?;

    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 3,
    };
    surface.configure(&device, &config);

    Ok((device, queue, surface))
}
