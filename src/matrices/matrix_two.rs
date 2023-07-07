use std::ops::*;

use crate::Vector2;


#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Matrix2 {
    pub x: Vector2,
    pub y: Vector2
}

impl Matrix2 {
    pub const IDENTITY: Matrix2 = Matrix2 {
        x: Vector2::X,
        y: Vector2::Y
    };

    pub const ONE: Matrix2 = Matrix2 {
        x: Vector2::ONE,
        y: Vector2::ONE
    };

    pub const EPSILON: Matrix2 = Matrix2 {
        x: Vector2::EPSILON,
        y: Vector2::EPSILON
    };

    pub fn new(
        r0c0: f32, r0c1: f32,
        r1c0: f32, r1c1: f32,
    ) -> Matrix2{
        Matrix2::from_rows([r0c0, r0c1], [r1c0, r1c1])
    }

    pub fn from_rows(
        x: impl Into<Vector2>,
        y: impl Into<Vector2>
    ) -> Matrix2 {
        Matrix2 {
            x: x.into(),
            y: y.into()
        }
    }

    pub fn c0(&self) -> Vector2 {
        Vector2::new(self.x.x, self.y.x)
    }

    pub fn c1(&self) -> Vector2 {
        Vector2::new(self.x.y, self.y.y)
    }

    pub fn det(&self) -> f32 {
        self.x.x * self.y.y - self.x.y * self.y.x
    }

    pub fn inverted(&self) -> Matrix2 {
        Matrix2::new(
            self.y.y, -self.x.y,
            -self.y.x, self.x.x
        ) / self.det()
    }
}

impl Mul for Matrix2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix2::new(
            self.x.dot(rhs.c0()), self.x.dot(rhs.c1()),
            self.y.dot(rhs.c0()), self.y.dot(rhs.c1()) 
        )
    }
}

impl Add for Matrix2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix2::from_rows(
            self.x + rhs.x,
            self.y + rhs.y
        )
    }
}

impl Sub for Matrix2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix2::from_rows(
            self.x - rhs.x,
            self.y - rhs.y
        )
    }
}


impl Mul<f32> for Matrix2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Matrix2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Mul<Vector2> for Matrix2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x.dot(rhs), self.y.dot(rhs))
    }
}


impl Div<f32> for Matrix2 {
    type Output = Matrix2;
    fn div(self, rhs: f32) -> Self::Output {
        Matrix2::from_rows(self.x / rhs, self.y / rhs)
    }
}