mod vec3;
use std::io::{self, Write};

use vec3::*;
fn main() {
    let image_width = 1024;
    let image_height = 1024;
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..=image_height - 1 {
        for i in 0..=image_width - 1 {
            let r = i as f64 / (image_height) as f64;
            let g = j as f64 / (image_width) as f64;
            let b = 0.0;

            println!(
                "{} {} {}",
                (r * 255.999) as u32,
                (g * 255.999) as u32,
                (b * 255.999) as u32
            );
        }
        eprint!("\r {}/{} lines rendered", j + 1, image_height)
    }
}
