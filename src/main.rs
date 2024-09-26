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
use rand::Rng;
use sphere::*;
use std::{f64::consts::PI, fs::File};
use vec3::*;
fn main() -> std::io::Result<()> {
    //World, or a very large list of all the objects in the scene.
    let mut world = HittableList {
        objects: Vec::new(),
    };
    /*

    let R = (PI/4.0).cos();

    let mat_right = Lambertian::new(Color::new(0., 0., 1.));
    let mat_left = Lambertian::new(Color::new(1., 0., 0.));

    world.add(Sphere::new(Point3::new(-R, 0., -1.), R, mat_left));
    world.add(Sphere::new(Point3::new(R, 0., -1.), R, mat_right));

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
    let mat_ground = Lambertian::new(Color::from(0.5));
    world.add(Sphere::new(Point3::new(0., -1000., 0.), 1000., mat_ground));
    let mut rng = rand::thread_rng();

    for a in (-11..11) {
        for b in (-11..11) {
            let choose_mat = rng.gen_range(-1.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(-1.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(-1.0..1.0),
            );

            if (center - vec3!(4, 0.2, 0)).length() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => {
                        // diffuse
                        let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                        let sphere_mat = Lambertian::new(albedo);
                        world.add(Sphere::new(center, 0.2, sphere_mat))
                    }
                    x if x < 0.95 => {
                        // metal
                        let albedo = Color::random(0.5, 1.);
                        let fuzz = rng.gen_range(0.0..0.5);
                        let sphere_mat = Metal::new(albedo, fuzz);
                        world.add(Sphere::new(center, 0.2, sphere_mat))

                    }
                    _ => {
                        // glass
                        let sphere_mat = Dielectric::new(1.5);
                        world.add(Sphere::new(center, 0.2, sphere_mat))

                    }
                }
            }
        }
    }

    let mat_1 = Dielectric::new(1.5);
    let mat_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let mat_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    world.add(Sphere::new(Point3::new(0., 1., 0.), 1., mat_1));
    world.add(Sphere::new(Point3::new(-4., 1., 0.), 1., mat_2));
    world.add(Sphere::new(Point3::new(4., 1., 0.), 1., mat_3));

    //Gets file from args
    let args: Vec<String> = std::env::args().collect();
    let file = File::create(args[1].to_string())?;

    let mut cam = Camera::new(file);
    //RENDER SETTINGS
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 600;
    cam.samples = 100;
    cam.bounces = 8;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(world)?;
    Ok(())
}
