mod color;
mod hittable;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod camera;

use hittable::*;
use sphere::*;
use vec3::*;
use camera::*;


fn main() {
  
    //World (how to change scene objects)
    let mut world = HittableList {
        objects: Vec::new(),
    };

    world.add(Sphere {
        center: Point3::new(0., -100.5, -1.),
        radius: 100.0,
    });
    world.add(Sphere {
        center: Point3::new(0., 0., -1.),
        radius: 0.5,
    });

        let mut cam = Camera::new();
            
        cam.aspect_ratio = 19.0/9.0;
        cam.image_width = 400;
        
        cam.render(world);

      }
