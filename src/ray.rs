//! This module contains the `Ray` struct, which represents a ray in 3D space.
use crate::vec3::*;
#[derive(Clone, Copy, Debug)]

/// A `Ray` is a struct that represents a ray in 3D space. It has an origin and a direction, represented by `Point3` and `Vec3` respectively.
pub struct Ray {
    /// The origin of the ray.
    pub origin: Point3,
    /// The direction of the ray.
    pub direction: Vec3,
}

impl Ray {
    /// Creates a new `Ray` with the given origin and direction.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
    /// Returns the point at a given distance `t` along the ray.
    pub fn at(self, t: f64) -> Point3 {
        return self.origin + (t * self.direction);
    }
}
