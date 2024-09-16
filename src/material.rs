use crate::{color::Color, ray::Ray, HitRecord, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        false
    }
}


pub struct Lambertian {
    albedo: Color,
}
pub struct Metal {
   albedo: Color,     
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }

    }
}
impl Lambertian {
 pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    
    }
}

impl Material for Lambertian {
    fn scatter(&self,r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_normalized();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {

        let  reflected = r_in.direction().reflect(&rec.normal);
        
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}

