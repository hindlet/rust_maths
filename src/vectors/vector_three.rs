#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::{Vector2, Vector4};
use super::super::Matrix3;
use std::{f32::consts::PI, cmp::Ordering, fmt::Display};
use std::f32::EPSILON;
use std::ops::*;
const HALF_PI: f32 = PI / 2.0;


/// Due to how the partialeq and partialord methods were auto implemented, I opted for a certain ordering for vector 3s
/// 
/// - If and only if the x components of the vectors are the same, the y components are checked
/// - If and only if the y components of the vectors are the same, the z components are checked
/// - If and only if the z components of the vectors are the same, they are equal
/// - If the values differ at any of these steps, the ordering of the components at that step is taken
/// 
#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Vector3 {
    pub x: f32, 
    pub y: f32,
    pub z: f32,
}


impl Vector3 {
    pub const X: Vector3 = Vector3{x: 1.0, y: 0.0, z: 0.0};
    pub const Y: Vector3 = Vector3{x: 0.0, y: 1.0, z: 0.0};
    pub const Z: Vector3 = Vector3{x: 0.0, y: 0.0, z: 1.0};
    pub const ZERO: Vector3 = Vector3{x: 0.0, y: 0.0, z: 0.0};
    pub const ONE: Vector3 = Vector3{x: 1.0, y: 1.0, z: 1.0};
    pub const EPSILON: Vector3 = Vector3{x: EPSILON, y: EPSILON, z: EPSILON};

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 {
            x,
            y,
            z,
        }
    }
    

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    pub fn normalise(&mut self){
        let length = self.magnitude();

        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn normalised(&self) -> Vector3 {
        let length = self.magnitude();

        Vector3::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn dot(&self, rhs: impl Into<Vector3>) -> f32{
        let rhs: Vector3 = rhs.into();
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: impl Into<Vector3>) -> Vector3 {
        let rhs: Vector3 = rhs.into();
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn outer_product(&self) -> [Vector3; 3]{
        [
            Vector3 {
                x: self.x * self.x,
                y: self.x * self.y,
                z: self.x * self.z,
            },
            Vector3 {
                x: self.y * self.x,
                y: self.y * self.y,
                z: self.y * self.z,
            },
            Vector3 {
                x: self.z * self.x,
                y: self.z * self.y,
                z: self.z * self.z,
            }
        ]
    }

    pub fn skew_symmetric(&self) -> [Vector3; 3] {
        [
            Vector3 {
                x: 0.0,
                y: -self.z,
                z: self.y,
            },
            Vector3 {
                x: self.z,
                y: 0.0,
                z: -self.x,
            },
            Vector3 {
                x: -self.y,
                y: self.x,
                z: 0.0,
            }
        ]
    }

    pub fn angle_to(&self, rhs: impl Into<Vector3>) -> f32 {
        let rhs: Vector3 = rhs.into();
        return (self.dot(rhs)/(self.magnitude() * rhs.magnitude())).acos();
    }

    pub fn xy(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    pub fn xz(&self) -> Vector2 {
        Vector2::new(self.x, self.z)
    }

    pub fn yz(&self) -> Vector2 {
        Vector2::new(self.y, self.z)
    }

    /// gets the appropriate euler angles for a given direction vector, where (0, 0, 0)euler is equivivelant to (0, 1, 0)direction
    pub fn direction_to_euler_angles(start_dir: impl Into<Vector3>) -> Vector3{
        let mut start_dir: Vector3 = start_dir.into();
        start_dir.normalise();
        if start_dir == Vector3::Y * -1.0 {
            Vector3::new(PI, 0.0, 0.0)
        } else if start_dir == Vector3::Y {
            Vector3::ZERO
        } else {
            let cross_mat = {
                let cross = start_dir.cross(Vector3::Y);
                Matrix3::new(
                    0.0, -cross.z, cross.y,
                    cross.z, 0.0, -cross.x,
                    -cross.y, cross.x, 0.0
                )
            };
            let angle_cos = start_dir.dot(Vector3::Y);
            let rot_mat =  Matrix3::IDENTITY + cross_mat + cross_mat * cross_mat * (1.0 / (1.0 + angle_cos));
            Matrix3::euler_angles_from(rot_mat)
        }

    }

    /// gets the approriate direction vector for given euler angles, where (0, 1, 0)direction is equivelant to (0, 0, 0)euler
    pub fn euler_angles_to_direction(rot: impl Into<Vector3>) -> Vector3 {
        let matrix = Matrix3::from_euler_angles(rot);
        matrix * Vector3::Y
    }

    /// returns the directions of the different components of a direction vector: 
    /// 
    /// e.g: (-7.0, 5.0, -1.0) -> (-1, 1, -1)
    pub fn direction_directions(&self) -> Vector3 {
        Vector3::new(self.x / self.x.abs(), self.y / self.y.abs(), self.z / self.z.abs())
    }

    pub fn sum(&self) -> f32 {
        self.x + self.y + self.z
    }

    pub fn floor(&self) -> Vector3 {
        Vector3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    pub fn to_isize_array(&self) -> [isize; 3] {
        [self.x.round() as isize, self.y.round() as isize, self.y.round() as isize]
    }

    pub fn extend(&self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, 0.0)
    }

    pub fn truncate(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl Eq for Vector3 {}


impl Ord for Vector3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (x, y, z): (i32, i32, i32);
        if (self.x - other.x).abs() < 1.17549435e-36f32 {
            x = 0;
        } else if self.x - other.x > 0.0 {
            x = 1;
        } else if self.x - other.x < 0.0 {
            x = -1;
        } else {
            x = 0;
        }

        if (self.y - other.y).abs() < 1.17549435e-36f32 {
            y = 0;
        } else if self.y - other.y > 0.0 {
            y = 1;
        } else if self.y - other.y < 0.0 {
            y = -1;
        } else {
            y = 0;
        }

        if (self.z - other.z).abs() < 1.17549435e-36f32 {
            z = 0;
        } else if self.z - other.z > 0.0 {
            z = 1;
        } else if self.z - other.z < 0.0 {
            z = -1;
        } else {
            z = 0;
        }

        if x > 0 {
            Ordering::Greater
        }
        else if x < 0 {
            Ordering::Less
        }
        else {
            if y > 0 {
                Ordering::Greater
            }
            else if y < 0 {
                Ordering::Less
            }
            else {
                if z > 0 {
                    Ordering::Greater
                }
                else if z < 0 {
                    Ordering::Less
                }
                else {
                    Ordering::Equal
                }
            }
        }
    }
}

//////////////////////////////////////////////////////////////////
///////////////////////////////// from and into
//////////////////////////////////////////////////////////////////

impl Into<[f32; 3]> for Vector3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}


impl From<[f32; 3]> for Vector3 {
    fn from(value: [f32; 3]) -> Self {
        Vector3::new(value[0], value[1], value[2])
    }
} 

impl From<[i32; 3]> for Vector3 {
    fn from(value: [i32; 3]) -> Self {
        Vector3::new(value[0] as f32, value[1] as f32, value[2] as f32)
    }
}

impl From<[u32; 3]> for Vector3 {
    fn from(value: [u32; 3]) -> Self {
        Vector3::new(value[0] as f32, value[1] as f32, value[2] as f32)
    }
}

impl From<[usize; 3]> for Vector3 {
    fn from(value: [usize; 3]) -> Self {
        Vector3::new(value[0] as f32, value[1] as f32, value[2] as f32)
    }
}
//////////////////////////////////////////////////////////////////
///////////////////////////////// arithmetic operations
//////////////////////////////////////////////////////////////////

impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Self;
    fn add(self, rhs: &Vector3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div for Vector3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}