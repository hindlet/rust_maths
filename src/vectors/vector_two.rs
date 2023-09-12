use std::ops::*;
use std::f32::EPSILON;
use super::Vector3;


#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f32, 
    pub y: f32,
}


impl Vector2 {
    pub const X: Vector2 = Vector2{x: 1.0, y: 0.0};
    pub const Y: Vector2 = Vector2{x: 0.0, y: 1.0};
    pub const ZERO: Vector2 = Vector2{x: 0.0, y: 0.0};
    pub const ONE: Vector2 = Vector2{x: 1.0, y: 1.0};
    pub const EPSILON: Vector2 = Vector2{x: EPSILON, y: EPSILON};

    pub fn new(x: f32, y: f32) -> Self {
        Vector2 {
            x,
            y,
        }
    }


    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }


    pub fn normalised(&self) -> Vector2{
        let length = self.magnitude();

        Vector2::new(
            self.x / length,
            self.y / length,
        )
    }

    pub fn normalise(&mut self){
        let length = self.magnitude();

        self.x /= length;
        self.y /= length;
    }


    pub fn dot(&self, rhs: impl Into<Vector2>) -> f32{
        let rhs: Vector2 = rhs.into();
        self.x * rhs.x + self.y * rhs.y
    }


    pub fn sum(&self) -> f32 {
        self.x + self.y
    }


    pub fn floor(&self) -> Vector2 {
        Vector2::new(self.x.floor(), self.y.floor())
    }


    pub fn to_isize_array(&self) -> [isize; 2] {
        [self.x.round() as isize, self.y.round() as isize]
    }

    pub fn extend(&self) -> Vector3 {
        Vector3::new(self.x, self.y, 0.0)
    }
}

//////////////////////////////////////////////////////////////////
///////////////////////////////// from and into
//////////////////////////////////////////////////////////////////

impl Into<[f32; 2]> for Vector2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}


impl From<[f32; 2]> for Vector2 {
    fn from(value: [f32; 2]) -> Self {
        Vector2::new(value[0], value[1])
    }
}

impl From<[i32; 2]> for Vector2 {
    fn from(value: [i32; 2]) -> Self {
        Vector2::new(value[0] as f32, value[1] as f32)
    }
}

//////////////////////////////////////////////////////////////////
///////////////////////////////// arithmetic operations
//////////////////////////////////////////////////////////////////


impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self{
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div for Vector2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self{
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Vector2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
