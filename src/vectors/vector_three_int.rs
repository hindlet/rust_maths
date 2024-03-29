use std::ops::*;
use serde::{Deserialize, Serialize};

use super::vector_three::Vector3;


#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Deserialize, Serialize)]
pub struct Vector3Int {
    pub x: i32, 
    pub y: i32,
    pub z: i32,
}

impl Vector3Int {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vector3Int {
            x, y, z
        }
    }

    pub const X: Vector3Int = Vector3Int{x: 1, y: 0, z: 0};
    pub const Y: Vector3Int = Vector3Int{x: 0, y: 1, z: 0};
    pub const Z: Vector3Int = Vector3Int{x: 0, y: 0, z: 1};
    pub const ZERO: Vector3Int = Vector3Int{x: 0, y: 0, z: 0};
    pub const ONE: Vector3Int = Vector3Int{x: 1, y: 1, z: 1};
}


//////////////////////////////////////////////////////////////////
///////////////////////////////// from and into
//////////////////////////////////////////////////////////////////

impl Into<[i32; 3]> for Vector3Int {
fn into(self) -> [i32; 3] {
    [self.x, self.y, self.z]
}
}


impl From<Vector3> for Vector3Int {
    fn from(value: Vector3) -> Self {
        Vector3Int::new(value.x.floor() as i32, value.y.floor() as i32, value.z.floor() as i32)
    }
}

impl From<[i32; 3]> for Vector3Int {
    fn from(value: [i32; 3]) -> Self {
        Vector3Int::new(value[0], value[1], value[2])
    }
}


//////////////////////////////////////////////////////////////////
///////////////////////////////// arithmetic operations
//////////////////////////////////////////////////////////////////

impl Mul<i32> for Vector3Int {
    type Output = Vector3Int;
    fn mul(self, rhs: i32) -> Self::Output {
        Vector3Int::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Add for Vector3Int {
    type Output = Vector3Int;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3Int::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3Int {
    type Output = Vector3Int;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3Int::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector3Int {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}