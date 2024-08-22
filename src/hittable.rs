use crate::{ray::Ray, vec3::*};
use std::ops::Range;
#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}


impl HitRecord {
    //Outward normal must be unit length
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.dir(), outward_normal) < 0.0;
        //rough translation, may cause errors
        match self.front_face == true {
            true => self.normal = *outward_normal,
            false => self.normal = -*outward_normal,
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::from(0.0),
            normal: Vec3::from(0.0),
            t: 0.0,
            front_face: false,
        }
    }
}

impl HittableList {
    pub fn add<T: Hittable +'static>(&mut self, object: T) {

        self.objects.push(Box::new(object));  }}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.end;

        for object in self.objects.iter() {
            if object.hit(r, ray_t.start..closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        false
    }
}
