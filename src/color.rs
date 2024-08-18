use crate::hittable::*;
use crate::ray::Ray;
use crate::sphere::*;
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

pub fn ray_color(r: Ray, world: &HittableList<Sphere>) -> Color {
    let mut rec: HitRecord = Default::default();

    if world.hit(&r, 0.0, f64::MAX, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1., 1., 1.));
    }

    let unit_direction = r.direction().normalized();
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0);
}
