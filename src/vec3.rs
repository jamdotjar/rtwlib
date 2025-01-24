//! `Vec3` is a simple 3D vector struct, with x, y, and z components, and a bunch of utility functions.
//! All of the vector math used in the raytracer is implemented here.
//! Vec3 has a few aliases, such as `Point3`, which is used to represent a point in 3D space, and `Color`, which is used to represent a color, and these are exchangable. ( althoug I would reccomend using `Color` for colors, and `Point3` for discrete positions )
use rand::Rng;
use std::ops::SubAssign;
#[allow(dead_code)]
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};
#[derive(Debug, Copy, Clone)]
///A 3D vector struct, with x, y, and z components, and a bunch of utility functions.
pub struct Vec3 {
    ///The x component of the vector.
    pub x: f64,
    ///The y component of the vector.
    pub y: f64,
    ///The z component of the vector.
    pub z: f64,
}
///An alias for Vec3, representing a point in 3D space, for convenience.
pub type Point3 = Vec3;

impl Vec3 {
    ///Creates a new Vec3 with the given x, y, and z components.
    pub fn new<T>(x: T, y: T, z: T) -> Self
    where
        T: Into<f64> + Copy,
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    /// Returns a random vector in the bounds of `min` to `max`
    /// All components are individually randomised
    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }
    /// returns the length of a vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    /// length_squared() - the length of a vector squared ( source of length )
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    /// Checks to see if a vector is very near to zero
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.x.abs() < s && self.y.abs() < s && self.z.abs() < s;
    }
    /// Returns a normalized vector with a length of 1
    pub fn normalized(self) -> Self {
        //constrains vectors to the unit sphere (-1 to 1)
        return self / self.length();
    }
    /// Returns a random vector within in the unit sphere
    fn random_in_unit_sphere() -> Vec3 {
        //gets a random normalized vector
        loop {
            let p = Vec3::random(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }
    /// Returns a random vector in the unit disk ( x and y components only )
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::random(-1., 1.) * Vec3::new(1., 1., 0.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }
    /// Reflects the vector across a surface normal
    pub fn reflect(self, n: &Vec3) -> Self {
        return self - *n * dot(&self, &n) * 2.;
    }
    /// Returns a random vector with length 1
    pub fn random_normalized() -> Vec3 {
        Self::random_in_unit_sphere().normalized()
    }
    /// Returns a random vector with length 1, on the hemisphere of a normal vector
    /// This means the vector will be facing outwards relative to the normal
    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        // a random vector facing outwards relative to the normal
        let on_sphere = Self::random_normalized();
        if dot(&on_sphere, &normal) > 0.0 {
            return on_sphere;
        } else {
            return -on_sphere;
        }
    }
}
#[macro_export]
///A macro for creating a new Vec3 with the given x, y, and z components.
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3 {
            x: $x as f64,
            y: $y as f64,
            z: $z as f64,
        }
    };
}
/// Returns the dot product of two vectors
/// The dot product is the sum of the products of the corresponding components of the two vectors.
///
/// dot of ( a, b ) = a.x * b.x + a.y * b.y + a.z * b.z
///
/// (1, 2, 3) dot (4, 5, 6) = 1*4 + 2*5 + 3*6 = 32
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    return (u[0] * v[0]) + (u[1] * v[1]) + (u[2] * v[2]);
}

/// Refracts a unit vector `uv` across a normal vector `n` with a given IOR (`etai_over_etat`) ratio.
/// This function is used to simulate the refraction of light through a material.
/// It returns a vector representing the direction refracted light.
pub fn refract(uv: Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(&-uv, n), 1.0); //max of two values
    let r_out_perp = etai_over_etat * (uv + cos_theta * *n);
    let r_out_parallel = -*n * (1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}
/// Returns the cross product of two vectors
/// The cross product is a vector that is perpendicular to the two input vectors.
///
/// cross of ( a, b ) = ( a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x )
///
/// (1, 2, 3) cross (4, 5, 6) = (2*6 - 3*5, 3*4 - 1*6, 1*5 - 2*4) = (-3, 6, -3)
pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

//A bunch of utility implementations for stuff.
impl From<f64> for Vec3 {
    fn from(n: f64) -> Self {
        Vec3 { x: n, y: n, z: n }
    }
}
//allows indexing the Vector with (unsigned) integer values, both mutable and immutable  eg. vec3[1] = vec3.y
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

//vec3 Neg implemntation, allows use of -vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl<T> Add<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Vec3;

    fn add(self, other: T) -> Vec3 {
        let other = other.into() as f64;
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl<T> Sub<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Vec3;

    fn sub(self, other: T) -> Vec3 {
        let other = other.into() as f64;
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Mul<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Vec3;

    fn mul(self, other: T) -> Vec3 {
        let other = other.into() as f64;
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl<T> Div<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Vec3;

    fn div(self, other: T) -> Vec3 {
        let other = other.into();
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}
impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}
