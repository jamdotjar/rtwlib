use std::ops::Range;
use std::rc::Rc;

use crate::utils::RangeExtensions;

use super::dot;
use super::HitRecord;
use super::Hittable;
use super::Material;

use super::Vec3;

#[derive(Clone, Debug)]
/// An infinite plane in 3D space, with an origin, normal, and material.
pub struct Plane {
    /// The origin of the plane
    pub origin: Vec3,
    /// The normal vector of the plane
    pub normal: Vec3,
    /// The material of the plane
    pub mat: Rc<dyn Material>,
}

impl Plane {
    /// Creates a new `Plane` with the given origin, normal and material.
    pub fn new(origin: Vec3, normal: Vec3, mat: Rc<dyn Material>) -> Self {
        Plane {
            origin,
            normal,
            mat,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        let denom = dot(&self.normal, &r.direction);
        if denom.abs() > 1e-4 {
            let t = dot(&(self.origin - r.origin), &self.normal) / denom;
            if ray_t.surrounds(t) {
                rec.t = t;
                rec.p = r.at(t);
                rec.set_face_normal(r, &self.normal);
                rec.set_material(Rc::clone(&self.mat));
                return true;
            }
        }
        false
    }
}
