use crate::vec3::*;
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn orig(&self) -> &Point3 {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3 {
        &self.direction
    }
    pub fn at(self, t: f64) -> Point3 {
        return self.origin + (t * self.direction);
    }
}
