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
//use rand::Rng;
use sphere::*;
use std::{f64::consts::PI, fs::File, rc::Rc};
use vec3::*;
fn main() -> std::io::Result<()> {
    //World, or a very large list of all the objects in the scene.
    let mut world = HittableList {
        objects: Vec::new(),
    };

    
    let mat_normal = Rc::new(Normal::new());
   // world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100.0, mat_normal.clone()));
    world.add(Sphere::new(Point3::new(0., 0., -1.2), 0.5, mat_normal.clone()));
  
    //Gets file from args
    let args: Vec<String> = std::env::args().collect();
    let file = File::create(args[1].to_string())?;


    let mut cam = Camera::new(file);
    //RENDER SETTINGS
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = args[2].parse().unwrap_or(600);
    
    cam.samples = 5;
    cam.bounces = 5;

    cam.vfov = 45.0;
    cam.lookfrom = Point3::new(0., 0., 1.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;
    cam.render(world)?;
    Ok(())
}
