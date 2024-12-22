use crate::{
    color::Color, material::{Lambertian, Material}, ray::Ray, sphere::Sphere, vec3::*
};

use std::{ops::Range, rc::Rc};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HitRecord {
    //Outward normal must be unit length
    pub fn set_face_normal<'a>(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        //rough translation, may cause errors
        match self.front_face == true {
            true => self.normal = *outward_normal,
            false => self.normal = -*outward_normal,
        }
    }

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
    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
    pub fn as_simple_vec(&self) -> Vec<String>  {
        let mut out = vec![];
        
            for object in &self.objects {
        out.push(object.as_string());
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

    fn as_string(&self) -> String{
       todo!() 
    }
}

pub trait Hittable {
    fn hit(&self, _r: &Ray, _ray_t: Range<f64>, _rec: &mut HitRecord) -> bool {
        false
    }
    fn as_string(&self) -> String; 
}

