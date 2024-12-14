use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    utils::RangeExtensions,
    vec3::*,
};
use std::{ops::Range, rc::Rc};
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius: f64::max(radius, 0.0),
            mat,
        }
    }
}


impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        //ray sphere interesctions
        let oc = self.center - r.origin;
        let a = &r.direction.length_squared();
        let h = dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 { //if it doesnt hit return false
            return false;
        }
        
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) { // make sure hit is in range
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        
        //callculates: 
        rec.t = root; //how far along the ray the hit was
        rec.p = r.at(rec.t); // hit point
        let outward_normal = (rec.p - self.center) / self.radius; //normals
        rec.set_face_normal(r, &outward_normal); //more specific normals
        rec.set_material(Rc::clone(&self.mat)); 

        return true;
    }
}
