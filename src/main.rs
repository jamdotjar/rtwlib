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

    
    let mat_floor = Rc::new(Lambertian::new(Color::from(1.)));
    let mat_l = Rc::new(Metal::new(Color::new(0.92, 0.9, 0.5), 0.));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 0.1));
    world.add(Sphere::new(Point3::new(0., -100.5, 0.), 100., mat_floor.clone()));
    world.add(Sphere::new(Point3::new(-0.5, 0., -1.2), 0.5, mat_left.clone()));
    world.add(Sphere::new(Point3::new(0.5, 0., -1.2), 0.5, mat_right.clone()));
  
    //Gets file from args
    let args: Vec<String> = std::env::args().collect();
    let file = File::create(args[1].to_string())?;


    let mut cam = Camera::new(file);
    //RENDER SETTINGS
    cam.aspect_ratio = 16.0 / 9.0;
    if args.len() > 2 {
    cam.image_width = args[2].parse().unwrap_or(600);}
    else {
        cam.image_width = 600;
    }
    
    cam.samples = 150;
    cam.bounces = 100;

    cam.vfov = 45.0;
    cam.lookfrom = Point3::new(0., 0., 1.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;
    cam.render(world)?;
    Ok(())
}
