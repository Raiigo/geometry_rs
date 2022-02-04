use std::ops::{Add, Mul};
use std::fmt::Display;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn scalar(&self, vector: &Self) -> f64 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
    }

    pub fn angle(&self, vector: &Self) -> f64 {
        ((self.scalar(vector)/(self.length() * vector.length()))).acos()
    }
}

impl<T: Into<f64> + Copy> Mul<T> for &Vec3
{
    type Output = Vec3;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs.into(),
            y: self.y * rhs.into(),
            z: self.z * rhs.into(),
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Into<f64> + Copy> From<[T; 3]> for Vec3 {
    fn from(list: [T; 3]) -> Self {
        Self {
            x: list[0].into(),
            y: list[1].into(),
            z: list[2].into(),
        }
    }
}

impl Display for &Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}