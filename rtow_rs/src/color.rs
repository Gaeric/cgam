use std::fs::File;
use std::io::Write;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(file: &mut File, pixel_color: &Color) {
    let r = (pixel_color.x()).clone();
    let g = (pixel_color.y()).clone();
    let b = (pixel_color.z()).clone();

    // apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let intensity = Interval::new(0.0, 0.999);

    let rbyte = (256.0 * intensity.clamp(r)) as u32;
    let gbyte = (256.0 * intensity.clamp(g)) as u32;
    let bbyte = (256.0 * intensity.clamp(b)) as u32;

    let _ = writeln!(file, "{rbyte} {gbyte} {bbyte}");
}
