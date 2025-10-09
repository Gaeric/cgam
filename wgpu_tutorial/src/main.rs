use wgpu_tutorial::run;

fn main() {
    pollster::block_on(run());

    println!("Hello, world!");
}
