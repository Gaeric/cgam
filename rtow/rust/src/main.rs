use core::time;
use std::{io::Write, thread::sleep};

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let ms = time::Duration::from_millis(10);

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {}", IMAGE_HEIGHT - j);
        std::io::stderr().flush().unwrap();
        sleep(ms);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let b = 0.0 as f64;

            let ir = (255.99 * r) as u32;
            let ig = (255.99 * g) as u32;
            let ib = (255.99 * b) as u32;

            println!("{ir} {ig} {ib}");
        }
    }

    eprintln!("\rDone.        ");
}
