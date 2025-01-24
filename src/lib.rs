#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![warn(missing_docs)]
//! # rtwlib
//! This is a simple and raytracing library, designed for simple use and modification, based on the books"Ray Tracing in One Weekend" and "Ray Tracing: the next week" by Peter Shirley.
//! This isn't optimized for performance, so I would heavily reccomend only using it in situations where computation time isn't a concern.
//! # Get started
//! To render a scene, you need two things: a `Camera` and a scene, in the form of a `HittableList`.
//! The following code creates a scene and camera, and renders the scene to a Vec<u8> containing the RGB values of the pixels.
//! ```
//! use rtwlib::{camera::Camera, hittable::HittableList};
//!
//! fn main() {
//!     let mut world = HittableList {
//!         objects: Vec::new(),
//!     };
//!     let mut cam = Camera::new();
//!
//!     let image_bytes = cam.render_to_bytes(world, |progress| println!("Progress: {}%", progress));
//!     // Do something with the bytes
//! }a
//! ```
//! This is the bare minimum needed to render a scene, and will result in a sky background, as no objects have been added to the scene.
//! ## Adding objects to the scene
//! Any object that implements the `Hittable` trait can be added to the scene, the base crate only provides a `Sphere` object that can be used, but you can create your own objects by implementing the `Hittable` trait for your own objects.
//! Objects also need to have an associated material, to inform how light bounces should be calculated which can be any object that implements the `Material` trait, this can be
//! ```
//! fn main() {
//!     let mut world = HittableList::new();
//!
//!     let material = Rc::new(Lambertian::new(Color::new(0.3, 0.86, 0.1))); //create a new material with a vaugely green color
//!     let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material); //create a new sphere at 0, 0, -1 with a radius of 0.5, using the material we just created
//!
//!     world.add(sphere); //add the sphere to the world
//!
//!     ...
//! //
pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod utils;
pub mod vec3;
