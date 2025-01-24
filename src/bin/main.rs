use plane::Plane;
use rtwlib::camera::*;
use rtwlib::color::Color;
use rtwlib::hittable::*;
use rtwlib::material::*;
//use rand::Rng;
use rtwlib::hittable::sphere::*;
use rtwlib::vec3::*;
use std::rc::Rc;
use std::{fs::File, io::Write};
fn main() -> std::io::Result<()> {
    //Create a world
    let mut world = HittableList {
        objects: Vec::new(),
    };
    //create the materials
    let mat_r = Rc::new(Lambertian::new(Color::new(1, 0, 0)));
    let mat_o = Rc::new(Lambertian::new(Color::new(1., 0.5, 0.)));
    let mat_y = Rc::new(Lambertian::new(Color::new(1., 1., 0.)));
    let mat_g = Rc::new(Lambertian::new(Color::new(0, 1, 0)));
    let mat_b = Rc::new(Lambertian::new(Color::new(0, 0, 1)));
    let mat_v = Rc::new(Lambertian::new(Color::new(0.8, 0.3, 0.8)));
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.9, 0.9, 0.9)));
    //add spheres to the world
    world.add(Sphere::new(Point3::new(0., 0., 2.5), 0.5, mat_r));
    world.add(Sphere::new(Point3::new(0., 0., 1.5), 0.5, mat_o));
    world.add(Sphere::new(Point3::new(0., 0., 0.5), 0.5, mat_y));
    world.add(Sphere::new(Point3::new(0., 0., -0.5), 0.5, mat_g));
    world.add(Sphere::new(Point3::new(0., 0., -1.5), 0.5, mat_b));
    world.add(Sphere::new(Point3::new(0., 0., -2.5), 0.5, mat_v));

    world.add(Plane::new(
        Point3::new(0., -0.5, 0.),
        Vec3::new(0., 1., 0.),
        mat_ground,
    ));
    //Creates a file based on args
    let args: Vec<String> = std::env::args().collect();
    let mut file = File::create(args[1].to_string())?;

    //create a new cam, and set relavant settings.
    let mut cam = Camera::new();
    cam.image_width = 300;
    cam.image_height = 150;
    cam.samples = 250;
    cam.bounces = 50;

    cam.vfov = 60.0;
    cam.lookfrom = Point3::new(3., 1.0, 0.);
    cam.lookat = Point3::new(0., 0.0, 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 2.3;
    cam.initialize(); //make sure to do this before render
    let lines = cam.image_height;

    //renders the image to a vector of bytes, running the progress callback every line.
    let buffer = cam.render_to_bytes(world, |progress| update_progress(progress, lines));

    println!("\n\rDone!");
    println!("Writing to {}...", args[1].to_string());

    //writes the buffer to a ppm
    file.write(format!("P6\n{} {}\n255\n", cam.image_width, cam.get_height()).as_bytes())?;
    file.write_all(&buffer)?;
    Ok(())
}

// Simple progress callback function
fn update_progress(progress: u32, lines: u32) {
    let strwidth = lines.to_string().len();
    print!(
        "\r{:0width$}/{} lines rendered",
        progress,
        lines,
        width = strwidth
    );
    std::io::stdout().flush().unwrap();
}
