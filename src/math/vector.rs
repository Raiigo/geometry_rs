use std::ops::{Add, Mul};

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

    pub fn scalar(&self, vector: &Self) -> f64 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
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