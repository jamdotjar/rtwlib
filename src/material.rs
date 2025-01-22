//! `materials` is a collection of types that implement the `Material` trait.
//! materials are associated with objects, and determine how it interacts with light
//! Every material has a `scatter` function, which takes an input ray and a hit record, and returns a boolean indicating if the ray was scattered, and modifies the input variables to reflect the scattered ray, colors, and other properties.
//! The available materials are:
//! - [`Lambertian`]: A diffuse material, effectively reflects light in a random direction, with a color determined by the albedo.
//! - [`Normal`]: A material that colors the object based on the normal vector at the hit point, mostly a joke, just a fancy colored lambertian.
//! - [`Metal`]: A material that reflects light. The reflectance is determined by the fuzziness of the material, with higher
use std::fmt::Debug;

use rand::Rng;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::*};

/// A `Material` is a trait that represents a material that can be applied to an object. This requires the `scatter` function to be implemented, which describes how the material scatters an incoming ray.
///
pub trait Material: Debug {
    /// Given an incoming ray and a hit record, this function should return a boolean indicating if the ray was scattered, and modify the input variables to reflect the scattered ray, colors, and other properties.
    /// # Arguments
    /// * `r_in` - The incoming ray
    /// * `rec` - A [`HitRecord`] ( stores location, normal, material,  and other information about the hit )
    /// * `attenuation` - The color of incoming light ray, to be modified by the material
    /// * `scattered` - The scattered ray, to be modified by the material
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    /// Returns a string representation of the material, for debugging purposes.
    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
/// A diffuse material, scatters light at random, with a color. It models a perfectly matte surface.
/// The `albedo` is the color of the material.
/// This has the most vibrarnt color of all the materials, as it reflects light in all directions.
pub struct Lambertian {
    albedo: Color,
}
#[derive(Debug)]
/// Almost Identical to the lambertian, but the color is dynamically determined by the normal vector at the hit point.
pub struct Normal {}
#[derive(Debug)]
/// A metal material, reflects light and imparts a slight color.
/// The reflectance is determined by the fuzziness of the material, with higher values being more blurry, don't use negative values unless you want some weird results.
/// The `albedo` is the color of the material, this generally looks like a tint of the reflected light.
pub struct Metal {
    albedo: Color,
    fuzz: f64, //I could enforce a specific range, buts its funnier not to.
}
#[derive(Debug)]
/// A dielectric material, refracts light, basically glass.
pub struct Dielectric {
    ior: f64,
}

impl Metal {
    /// Creates a new `Metal` material with the given albedo and fuzziness.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}
impl Lambertian {
    /// Creates a new `Lambertian` material with the given albedo.
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}
impl Dielectric {
    /// Creates a new `Dielectric` material with the given index of refraction.
    pub fn new(ior: f64) -> Self {
        Dielectric { ior }
    }
}
impl Normal {
    /// Creates a new `Normal` material.
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
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + (Vec3::random_normalized());
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
        let direction: Vec3;
        if cannot_refract || reflectance(cos_theta, ri) > rand::thread_rng().gen_range(0.0..1.0) {
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
