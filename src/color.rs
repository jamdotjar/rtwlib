use core::f64;

use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;
pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    //convert colors from 0-1 f64 range to 8 bit integer (0-255)
    let rbyte = (255.999 * r) as u8;
    let gbyte = (255.999 * g) as u8;
    let bbyte = (255.999 * b) as u8;

    println!("{} {} {}\n", rbyte, gbyte, bbyte)
}


