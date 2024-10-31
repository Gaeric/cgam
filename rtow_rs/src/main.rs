use core::time;
use std::{io::Write, thread::sleep};

use color::{write_color, Color};

mod camera;
mod color;
mod interval;
mod ray;
mod vec3;
mod hittable;
mod material;

const DELAY_TIME: u64 = 0;

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let ms = time::Duration::from_millis(DELAY_TIME);

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {}", IMAGE_HEIGHT - j);
        std::io::stderr().flush().unwrap();
        sleep(ms);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let b = 0.0 as f64;

            let pixel_color = Color::new(r, g, b);

            write_color(&pixel_color);
        }
    }

    eprintln!("\rDone.        ");
}
