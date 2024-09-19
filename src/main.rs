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
use vec3::*;

fn main() {
    //World, or a very large list of all the objects in the scene. 
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let mat_left = Metal::new(Color::from(0.8), 0.);
    let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.8);
    world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100.0, mat_ground));
    world.add(Sphere::new(Point3::new(0., 0., -1.2), 0.5, mat_center));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), 0.5, mat_left));
    world.add(Sphere::new(Point3::new(1., 0., -1.), 0.5, mat_right));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples = 100;
    cam.bounces = 50;

    cam.render(world);
}
