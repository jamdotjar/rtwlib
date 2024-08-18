use crate::{
    hittable::{HitRecord, Hittable},
    vec3::*,
};

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius: f64::max(radius, 0.0),
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = &r.direction.length_squared();
        let h = dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;
        if (root <= ray_tmin || ray_tmax <= root) {
            let root = (h + sqrtd) / a;
            if (root <= ray_tmin || ray_tmax <= root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        return true;
    }
}
