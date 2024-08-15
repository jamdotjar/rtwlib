mod color;
mod ray;
mod vec3;
use std::io::{self, Write};

use color::*;
use ray::*;
use vec3::*;
fn main() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..=image_height - 1 {
        for i in 0..=image_width - 1 {
            let pixel_color = Color {
                x: i as f64 / (image_height) as f64,
                y: j as f64 / (image_width) as f64,
                z: 0.0,
            };

            write_color(pixel_color);
        }
        eprint!("\r {}/{} lines rendered", j + 1, image_height)
    }
}
