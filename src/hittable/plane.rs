//! A module for the `Plane` struct and its implementation.
//! A `Plane` is an infinite plane, defined by an origin point, a normal vector, and a material.
//! The normal vector is the direction that the plane faces.
use std::ops::Range;
use std::rc::Rc;

use crate::utils::RangeExtensions;

use super::dot;
use super::HitRecord;
use super::Hittable;
use super::Material;

use super::Vec3;

#[derive(Clone, Debug)]
/// An infinite plane, with an origin, normal, and material.
/// The normal
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
            normal: normal.normalized(),
            mat,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        // this took me like 2 hours, dont screw around with it too much.
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
    fn as_string(&self) -> String {
        format!(
            "[ Plane ] Normal: ({}, {}, {}), Position: ({}x, {}z, {}z), material: {:?}",
            self.normal.x,
            self.normal.y,
            self.normal.z,
            self.origin.x,
            self.origin.y,
            self.origin.z,
            self.mat
        )
    }
    fn as_info_vec(&self) -> Vec<String> {
        vec![
            "Plane".to_string(),
            "∞".to_string(),
            self.origin.x.to_string(),
            self.origin.y.to_string(),
            self.origin.z.to_string(),
            format!("{:?}", self.mat),
        ]
    }
}
