#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::{Vector2, Vector3};
use std::{f32::EPSILON, fmt::Display};
use std::ops::*;


#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Vector4 {
    pub x: f32, 
    pub y: f32,
    pub z: f32,
    pub w: f32,
}



impl Vector4 {
    pub const X: Vector4 = Vector4{x: 1.0, y: 0.0, z: 0.0, w: 0.0};
    pub const Y: Vector4 = Vector4{x: 0.0, y: 1.0, z: 0.0, w: 0.0};
    pub const Z: Vector4 = Vector4{x: 0.0, y: 0.0, z: 1.0, w: 0.0};
    pub const W: Vector4 = Vector4{x: 0.0, y: 0.0, z: 0.0, w: 1.0};
    pub const ZERO: Vector4 = Vector4{x: 0.0, y: 0.0, z: 0.0, w: 0.0};
    pub const ONE: Vector4 = Vector4{x: 1.0, y: 1.0, z: 1.0, w: 1.0};
    pub const EPSILON: Vector4 = Vector4{x: EPSILON, y: EPSILON, z: EPSILON, w: EPSILON};

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 {
            x,
            y,
            z,
            w,
        }
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    pub fn normalise(&mut self){
        let length = self.magnitude();

        self.x /= length;
        self.y /= length;
        self.z /= length;
        self.w /= length;
    }

    pub fn dot(&self, rhs: impl Into<Vector4>) -> f32{
        let rhs = rhs.into();
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }


    pub fn xy(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y
        }
    }

    pub fn xz(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.z
        }
    }

    pub fn yz(&self) -> Vector2 {
        Vector2 {
            x: self.y,
            y: self.z
        }
    }

    pub fn sum(&self) -> f32 {
        self.x + self.y + self.z + self.w
    }

    pub fn truncate(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn truncate_n(&self, n: usize) -> Vector3 {
        match n {
            0 => Vector3::new(self.y, self.z, self.w),
            1 => Vector3::new(self.x, self.z, self.w),
            2 => Vector3::new(self.x, self.y, self.w),
            3 => Vector3::new(self.x, self.y, self.z),
            _ => panic!("Index out of range")
        }
    }
}

impl Into<[f32; 4]> for Vector4 {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl From<[f32; 4]> for Vector4 {
    fn from(value: [f32; 4]) -> Self {
        Vector4::new(value[0], value[1], value[2], value[3])
    }
}

// arithmetic ops

impl Add for Vector4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vector4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl SubAssign for Vector4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<f32> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f32> for Vector4 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Neg for Vector4 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Display for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}