# rtwlib
[![Crates.io](https://img.shields.io/crates/v/rtwlib?style=flat-square)](https://crates.io/crates/rtwlib)
[![Crates.io](https://img.shields.io/crates/d/rtwlib?style=flat-square)](https://crates.io/crates/rtwlib)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](./LICENSE)
[![Crates.io Size](https://img.shields.io/crates/size/rtwlib)](https://crates.io/crates/rtwlib)
[![docs.rs](https://img.shields.io/docsrs/rtwlib)](https://docs.rs/rtwlib/latest/rtwlib)

This is a simple and raytracing library, designed for simple use and modification, based on the books"[raytracting in one weekend](https://github.com/RayTracing)" and "Ray Tracing: the next week" by Peter Shirley. This isn't optimized for performance, so I would heavily reccomend only using it in situations where computation time isn't a concern.

This *is* a library, so it can be used in other projects. If you want to just mess around with the raytracer, you can use the main.rs file, which has some basic examples. If you dont want to mess with the source code, my project, [rtw.tui](https://github.com/jamdotjar/rtweekend-tui) lets you create and render scenes with a simple terminal interface.


This is mainly to learn about raytracing and rust Structs, Impl and Traits, my goal is to try and implement as much of the required functionality by hand. Right now, this is mainly just the `Vec3` class and associated functions, But I plan to add a homemade random number generator, and a PNG encoder. 
## Features:
- Simple and easy to use
- Customizable
- Supports multiple materials
- Spheres
- Planes
- [Semi-readable documentation](https://docs.rs/rtwlib/latest/rtwlib/)

## Usage:

To use this library, add the following to your Cargo.toml file:

```toml
[dependencies]
rtwlib = "0.1.2"
```

To render a scene, you need a `Camera` and a scene in the form of a `HittableList`. The following code creates a scene and camera, and renders the scene to a `Vec<u8>` containing the RGB values of the pixels.

```rust
use rtwlib::{camera::Camera, hittable::HittableList};

fn main() {
    let mut world = HittableList {
        objects: Vec::new(),
    };
    let mut cam = Camera::new();

    let image_bytes = cam.render_to_bytes(world, |progress| println!("Progress: {}%", progress));
    // Do something with the bytes
}
```

This is the bare minimum needed to render a scene and will result in a sky background, as no objects have been added to the scene.

## Adding objects to the scene

Any object that implements the `Hittable` trait can be added to the scene. The base crate provides a `Sphere` object, but you can create your own objects by implementing the `Hittable` trait. Objects also need an associated material to inform how light bounces should be calculated, which can be any object that implements the `Material` trait.

```rust
fn main() {
    let mut world = HittableList::new();

    let material = Rc::new(Lambertian::new(Color::new(0.3, 0.86, 0.1))); // Create a new material with a vaguely green color
    let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material); // Create a new sphere at (0, 0, -1) with a radius of 0.5, using the material we just created

    world.add(sphere); // Add the sphere to the world

    // ...
}
```
## Full example

```rust
use rtwlib::camera::*;
use rtwlib::color::Color;
use rtwlib::hittable::*;
use rtwlib::material::*;
use rtwlib::sphere::*;
use rtwlib::vec3::*;
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();

    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right));

    let mut cam = Camera::new();
    cam.image_width = 400;
    cam.image_height = 225;
    cam.samples = 100;
    cam.bounces = 50;

    let image_bytes = cam.render_to_bytes(world, |progress| println!("Progress: {}%", progress));
    // Do something with the bytes, e.g., save to a file
}
```
for further examples, see the [`examples`](./src/examples/)directory.

## Tips and tricks
If you want to save to a file, you easily render to a ppm like so:
```rust
 let buffer = cam.render_to_bytes(world, |progress| update_progress(progress, lines));

println!("\n\rDone!");
println!("Writing to {}...", args[1].to_string());
let mut file = File::create("output.ppm")?;

file.write(format!("P6\n{} {}\n255\n", cam.image_width, cam.get_height()).as_bytes())?;
file.write_all(&buffer)?;
Ok(())
```

You can kind of fake emmissive materials by setting the color values of a material to be greater than 1.0, or mulyiplying the base color by some large-ish number.

```rust
let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.8)*2));
```

## Gallery
![dof2](https://github.com/user-attachments/assets/d5495b1c-87dd-4df4-a8b3-179291c67830)
![diffuse](https://github.com/user-attachments/assets/5ed2b24a-ddb1-4130-a565-9b0873236bae)
![image](https://github.com/user-attachments/assets/339355a2-5a0f-439e-8756-4570e9a0622f)
![final33](https://github.com/user-attachments/assets/e125ecf3-bb8c-44e7-89d6-415e7985444c)


## Future plans:
My general goal is to similtanouisly develop this, and the tui, to "test" the library. Some feature I want to add are:
- More materials
- More shapes
- Multithreading
- More object types
- BVH optimization
- PNG handling export support.
You can also probably assume most of the second book will be implemented at some point.
