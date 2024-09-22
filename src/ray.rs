use crate::vec3::*;
#[derive(Clone, Copy, Debug)]

//ray, a vector with an arbitray origin ig
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        return self.origin + (t * self.direction);
    }
}
