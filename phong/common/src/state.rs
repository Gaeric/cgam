use winit::{event::WindowEvent, window::Window};

pub trait State {
    fn window(&self) -> &Window;
    fn update(&mut self);
    fn size(&mut self) -> winit::dpi::PhysicalSize<u32>;
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    fn input(&mut self, event: &WindowEvent) -> bool;
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}
