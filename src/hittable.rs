//! `Hittable` is a trait that is used to define objects that can be hit by rays, and `HittableList` represents a list of `Hittable` objects, or a scene.
//! This module contains the hittable trait, and the hittable list struct, which is a collection of hittable objects.
//! Any hittable object must implement the `Hittable` trait, which requires the `hit` function to be implemented, which determines if a ray hits the object.
//! The `HittableList` struct is a collection of hittable objects, and implements the `Hittable` trait itself, allowing for nested collections of objects ( I dont see why you would need that ).
//!
use crate::{
    color::Color,
    material::{Lambertian, Material},
    ray::Ray,
    vec3::*,
};

use std::{ops::Range, rc::Rc};

/// A `HitRecord` is a struct that contains information about a hit, such as the hit point, normal, material, and other information.
pub struct HitRecord {
    /// The location of the hit
    pub p: Point3,
    /// The normal vector at the hit point
    pub normal: Vec3,
    /// The material of the object that was hit
    pub mat: Rc<dyn Material>,
    /// The distance along the ray that the hit was
    pub t: f64,
    /// A boolean indicating if the hit was on the front face of the object
    pub front_face: bool,
}
/// A `HittableList` is a struct that contains a list of `Hittable` objects, and implements the `Hittable` trait itself. Mostly useful to quickly test all objects in a scene for hits. Use it for scenes. idk
/// # Example
/// ```
/// use rtwlib::hittable::HittableList;
/// use rtwlib::sphere::Sphere;
/// use rtwlib::vec3::Point3;
/// use rtwlib::material::Lambertian;
///
/// let mut world = HittableList::new();
/// let material = Rc::new(Lambertian::new(Color::from(0.3)));
/// let sphere = Sphere::new(Point3::from(0.0), 0.5, material);
/// world.add(sphere);
pub struct HittableList {
    /// A list of hittable objects, stored on the heap
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HitRecord {
    /// Sets the normal of the hit record based on the outward normal and the ray direction, the outward normal must be a unit vector.
    pub fn set_face_normal<'a>(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        //rough translation, may cause errors
        match self.front_face == true {
            true => self.normal = *outward_normal,
            false => self.normal = -*outward_normal,
        }
    }
    /// Sets the material of the hit record
    pub fn set_material(&mut self, mat: Rc<dyn Material>) {
        self.mat = mat;
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::from(0.0),
            normal: Vec3::from(0.0),
            mat: Rc::new(Lambertian::new(Color::from(0.5))),
            t: 0.0,
            front_face: false,
        }
    }
}

impl HittableList {
    /// Creates a new `HittableList` with an empty list of objects.
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    /// Adds an object to the `HittableList`.
    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
    /// Returns a vector of strings representing the objects in the list.
    pub fn as_simple_vec(&self) -> Vec<String> {
        let mut out = vec![];

        for object in &self.objects {
            out.push(object.as_string());
        }

        out
    }
    /// Returns a vector of vectors of strings representing the objects in the list
    pub fn as_info_vec(&self) -> Vec<Vec<String>> {
        let mut out = vec![];

        for object in &self.objects {
            out.push(object.as_info_vec());
        }

        out
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.end;

        for object in self.objects.iter() {
            //checks every object for a hit
            let mut temp_rec: HitRecord = Default::default();

            if object.hit(r, ray_t.start..closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
}
/// The `Hittable` trait is used to define objects that can be hit by rays, it would be implented by any object in a scene like a Sphere or Cube.
pub trait Hittable: HittableClone {
    /// Determines if a ray hits the object, and modifies a [`HitRecord`] if it does.
    fn hit(&self, _r: &Ray, _ray_t: Range<f64>, _rec: &mut HitRecord) -> bool {
        false
    }
    /// Returns a string representation of the object.
    fn as_string(&self) -> String {
        "Hittable".to_string()
    }
    /// Returns a vector of strings representing the object.
    fn as_info_vec(&self) -> Vec<String> {
        vec![]
    }
}
/// A trait to allow cloning of `Hittable` objects.
pub trait HittableClone {
    /// Clones the object as a boxed trait object.
    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl<T> HittableClone for T
where
    T: 'static + Hittable + Clone,
{
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

impl Clone for HittableList {
    fn clone(&self) -> Self {
        HittableList {
            objects: self.objects.iter().map(|obj| obj.clone()).collect(),
        }
    }
}
