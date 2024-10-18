use winit::{event_loop::EventLoop, window::WindowBuilder};

use rtr_tutorial::PhongState;

async fn exec() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut state = PhongState::new(&window).await.unwrap();
    common::run(&mut state, event_loop).await;
}

fn main() {
    pollster::block_on(exec());
}
