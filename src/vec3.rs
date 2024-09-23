#[allow(dead_code)]
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub type Point3 = Vec3;

impl Vec3 {
    //new(x,y,z)
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    //returns a random vector in the bounds of min to max
    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }
    //length() - returns the length of a vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    //length_squared() - the length of a vector squared ( source of length )
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool { 
        let s = 1e-8;
        return self.x.abs() < s && self.y.abs() < s && self.z.abs() < s;
    }

    pub fn normalized(self) -> Self { //constrains vectors to the unit sphere (-1 to 1)
        return self / self.length();
    } 

    fn random_in_unit_sphere() -> Vec3 { //gets a random normalized vector
        loop {
            let p = Vec3::random(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn reflect(self, n: &Vec3) -> Self {
        return self - *n * dot(&self, &n) * 2.;
    }
    


    pub fn random_normalized() -> Vec3 {
        Self::random_in_unit_sphere().normalized()
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 { // a random vector facing outwards relative to the normal
        let on_sphere = Self::random_normalized();
        if dot(&on_sphere, &normal) > 0.0 {
            return on_sphere;
        } else {
            return -on_sphere;
        }
    }
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3 {
            x: $x as f64,
            y: $y as f64,
            z: $z as f64,
        }
    };
}
use rand::Rng;
pub(crate) use vec3;

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    return (u[0] * v[0]) + (u[1] * v[1]) + (u[2] * v[2]);
}
    pub fn refract(uv: Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(dot(& -uv, n), 1.0); //max of two values
        let r_out_perp = etai_over_etat * ( uv + cos_theta * *n);
        let r_out_parallel = -*n * (1.0-r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
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

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
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

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
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
