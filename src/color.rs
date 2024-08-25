use crate::{utils::RangeExtensions, vec3::*};

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let intensity = (0.0, 0.999);
    //convert colors from 0-1 f64 range to 8 bit integer (0-255)
    let rbyte =(  r.clamp(intensity.0, intensity.1)*255.0)as u8;
    let gbyte =(  g.clamp(intensity.0, intensity.1)*255. )as u8;
    let bbyte =(  b.clamp(intensity.0, intensity.1)*255. )as u8;


    println!("{} {} {}\n", rbyte, gbyte, bbyte)
}


