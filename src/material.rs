use crate::{color::Color, dot, ray::Ray, vec3::*, HitRecord};

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
pub struct Metal {
    albedo: Color,
    fuzz: f64, //I could enforce a specific range, buts its funnier not to.
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
