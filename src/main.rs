mod color;
mod ray;

mod vec3;

use color::*;
use ray::*;
use std::io::{self, Write};
use vec3::*;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin;
    let a = r.direction.length_squared();
    let h = dot(&r.direction, &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(vec3!(0, 0, -1), 0.5, &r);
    if t > 0.0 {
        let N = (r.at(t) - vec3!(0, 0, -1));
        return 0.5 * Color::new(N.x + 1., N.y + 1., N.z + 1.);
    }

    let unit_direction = r.direction().normalized();
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    //image size
    let aspect_ratio = (16.0 / 9.0);
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    //camera and viewport
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    //Creates the vectors across the edge of the viewport
    let viewport_u = vec3!(viewport_width, 0, 0);
    let viewport_v = vec3!(0, -viewport_height, 0);

    //calculates the delta vectors, for movement between pixels.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - vec3!(0, 0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..=image_height - 1 {
        for i in 0..=image_width - 1 {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray {
                origin: camera_center,
                direction: ray_direction,
            };

            let pixel_color: Color = ray_color(r);
            write_color(pixel_color);
        }
        eprint!("\r {}/{} lines rendered", j + 1, image_height)
    }
}
