use algebra::Vec3;
use anyhow::{Context, Result};
use camera::Camera;
use winit::event::{DeviceEvent, Event, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ControlFlow, DeviceEvents, EventLoop};
use winit::window::{Window, WindowBuilder};

mod algebra;
mod camera;
mod render;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[pollster::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.listen_device_events(DeviceEvents::Always);
    let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_resizable(false)
        .with_title("GPU PT".to_string())
        .build(&event_loop)?;

    let (device, queue, surface) = connect_to_gpu(&window).await?;
    let mut renderer = render::PathTracer::new(device, queue, WIDTH, HEIGHT);
    let mut camera = Camera::look_at(
        Vec3::new(0.0, 0.75, 1.0),
        Vec3::new(0.0, -0.5, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    event_loop.run(|event, control_handle| {
        control_handle.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_handle.exit(),
                WindowEvent::RedrawRequested => {
                    // Wait for the next available frame buffer
                    let frame: wgpu::SurfaceTexture = surface
                        .get_current_texture()
                        .expect("failed to get current texture");

                    let render_target = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    renderer.render_frame(&camera, &render_target);

                    frame.present();
                    window.request_redraw();
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let delta = match delta {
                        MouseScrollDelta::PixelDelta(delta) => 0.001 * delta.y as f32,
                        MouseScrollDelta::LineDelta(_, y) => y * 0.1,
                    };
                    camera.zoom(-delta);
                }
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseWheel { delta } => {
                    let delta = match delta {
                        MouseScrollDelta::PixelDelta(delta) => 0.001 * delta.y as f32,
                        MouseScrollDelta::LineDelta(_, y) => y * 0.1,
                    };
                    camera.zoom(delta);
                }
                _ => (),
            },
            _ => (),
        }
    })?;

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
        .find(|it| matches!(it, wgpu::TextureFormat::Bgra8Unorm))
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
