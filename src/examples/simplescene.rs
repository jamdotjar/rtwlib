use rtwlib::camera::*;
use rtwlib::color::Color;
use rtwlib::hittable::*;
use rtwlib::material::*;
use rtwlib::sphere::*;
use rtwlib::vec3::*;
use std::rc::Rc;
use std::{fs::File, io::Write};
fn main() -> std::io::Result<()> {
    //Create a scene
    let mut world = HittableList::new();

    //Create materials, for the scene objects
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.3, 0.86, 0.1)));
    let mat_center = Rc::new(Normal::new());
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.2));
    /// Add spheres to the world
    world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100.0, mat_ground)); // A large sphere to act as the ground
    world.add(Sphere::new(Point3::new(0., 0., -1.), 0.5, mat_center));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), 0.5, mat_left));
    world.add(Sphere::new(Point3::new(1., 0., -1.), 0.5, mat_right));

    //Create a new camera
    let mut cam = Camera::new();
    // Set camera settings
    //resolution
    cam.image_width = 1600;
    cam.image_height = 900;
    //quality
    cam.samples = 250;
    cam.bounces = 50;
    //position
    cam.lookfrom = Point3::new(1.5, 0.5, 0.);
    cam.lookat = Point3::new(0., 0., -0.5);
    cam.vup = Vec3::new(0., 1., 0.);
    //visual settings
    cam.vfov = 45.0;
    cam.defocus_angle = 2.;
    cam.focus_dist = cam.get_distance(cam.lookat); // set the focus distance to the distance between the camera and the lookat point

    //renders the image to a vector of bytes, running the progress callback every line.
    let buffer = cam.render_to_bytes(world, |progress| println!("Progress: {}%", progress / 900));
    println!("\n\rDone!");
    println!("Writing to output.ppm");

    //writes the buffer to a ppm

    let mut file = File::create("output.ppm")?;
    file.write(format!("P6\n{} {}\n255\n", cam.image_width, cam.get_height()).as_bytes())?;
    file.write_all(&buffer)?;
    Ok(())
}
