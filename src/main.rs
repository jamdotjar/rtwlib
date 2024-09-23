mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::*;
use color::Color;
use hittable::*;
use material::*;
use sphere::*;
use std::{f64::consts::PI, fs::File};
use vec3::*;
fn main() -> std::io::Result<()> {
    //World, or a very large list of all the objects in the scene.
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let R = (PI/4.0).cos();

    let mat_right = Lambertian::new(Color::new(0., 0., 1.));
    let mat_left = Lambertian::new(Color::new(1., 0., 0.));

    world.add(Sphere::new(Point3::new(-R, 0., -1.), R, mat_left));
    world.add(Sphere::new(Point3::new(R, 0., -1.), R, mat_right));
    /*
    let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let mat_left = Dielectric::new(1.5);
    let mat_bubble = Dielectric::new(1.0/1.5);
    let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.2);
    world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100.0, mat_ground));
    world.add(Sphere::new(Point3::new(0., 0., -1.2), 0.5, mat_center));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), 0.5, mat_left));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), 0.4, mat_bubble));
    world.add(Sphere::new(Point3::new(1., 0., -1.), 0.5, mat_right));
    */
    //Gets file from args
    let args: Vec<String> = std::env::args().collect();
    let file = File::create(args[1].to_string())?;

    let mut cam = Camera::new(file);
    //RENDER SETTINGS
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 600;
    cam.samples = 100;
    cam.bounces = 8;

    cam.vfov = 90.0;


    cam.render(world)?;
    Ok(())
}
