use rand::{Rng};

use crate::{color::Color, ray::Ray, vec3::*, hittable::HitRecord};

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Color,
}
pub struct Normal {
}
pub struct Metal {
    albedo: Color,
    fuzz: f64, //I could enforce a specific range, buts its funnier not to.
}
pub struct Dielectric {
    ior: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}
impl Dielectric {
    pub fn new(ior: f64) -> Self {
        Dielectric { ior }
    
    }
   }
impl Normal {
    pub fn new() -> Self {
        Normal {}
    }    
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + (Vec3::random_normalized()); //on hit, send the ray in a random direction ( on the surface of the sphere )

        //Checks to make sure the direction isnt too close to 0, which causes artfacting
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        *scattered = Ray::new(rec.p, scatter_direction); //send a new ray in the sactter direction
                                                         //from from hitpoint (rec.p)
        *attenuation = self.albedo;
        true
    }
}

impl Material for Normal {
    fn scatter(&self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,) -> bool {
            let mut scatter_direction = rec.normal + (Vec3::random_normalized());
            *scattered = Ray::new(rec.p, scatter_direction);
            *attenuation = Color::new(rec.normal.x, rec.normal.y, rec.normal.z);
            true
        }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = r_in.direction.reflect(&rec.normal);
        let reflected = reflected.normalized() + Vec3::random_normalized() * self.fuzz;

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return dot(&scattered.direction, &rec.normal) > 0.;
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1., 1., 1.);

        let ri: f64 = if rec.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = r_in.direction.normalized();
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = ri * sin_theta > 1.0;
        let mut direction = Vec3::from(0.);
        
        
        if cannot_refract || reflectance(cos_theta, ri) > rand::thread_rng().gen_range(0.0..1.0){
            direction = unit_direction.reflect(&rec.normal)
        } else {
            direction = refract(unit_direction, &rec.normal, ri)
        }

        *scattered = Ray::new(rec.p, direction);

        true
    }
}

//schlick approximation for reflectance at grazing angles
 fn reflectance(cos: f64, ior: f64) -> f64 {
        let r0 = (1. - ior) / (1. + ior);
        let r0 = r0 * r0; //if everything breaks again try changing this
        r0 + (1. - r0) * (1. - cos).powf(5.)
    }

