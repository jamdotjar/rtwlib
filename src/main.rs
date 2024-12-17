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

    
    let mat_floor = Rc::new(Lambertian::new(Color::new(0.1, 0.9, 0.3)));
    let mat_back = Rc::new(Lambertian::new(Color::new(1., 0.1, 0.2)));
    let mat_back2 = Rc::new(Metal::new(Color::new(0.8, 0.7, 0.6), 0.0));
    let mat_front = Rc::new(Dielectric::new( 1.47));
    let mat_front2 = Rc::new(Dielectric::new( 1.0/1.5));
    world.add(Sphere::new(Point3::new(0., -100.5, 0.), 100., mat_floor.clone()));
    world.add(Sphere::new(Point3::new(0.7, 0.2, -2.), 0.7, mat_back.clone()));
    world.add(Sphere::new(Point3::new(-0.7, 0.2, -2.), 0.7, mat_back2.clone()));
    world.add(Sphere::new(Point3::new(0., 0., -1.), 0.5, mat_front.clone()));
    //world.add(Sphere::new(Point3::new(-0.1, 0., -1.2), 0.1, mat_front2.clone()));
  
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
    
    cam.samples = 500;
    cam.bounces = 100;

    cam.vfov = 45.0;
    cam.lookfrom = Point3::new(0., 0., 1.);
    cam.lookat = Point3::new(0., 0., -1.2);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.0 ;
    cam.focus_dist = 10.0;
    print!("rendering scene with {} objects 
    resolution: {}x{}
    samples: {}
    bounces: {}
    fov: {}",
     world.objects.len(), cam.image_width, (cam.image_width as f64 /cam.aspect_ratio) as u32, cam.samples, cam.bounces, cam.vfov);
    cam.render(world)?;
    Ok(())
}
