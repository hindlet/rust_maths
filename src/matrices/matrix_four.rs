#![allow(dead_code)]
use super::super::vectors::Vector4;
use super::*;
use std::ops::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Matrix4 {
    pub x: Vector4, 
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4 {
        x: Vector4::X,
        y: Vector4::Y,
        z: Vector4::Z,
        w: Vector4::W
    };

    pub const ONE: Matrix4 = Matrix4 {
        x: Vector4::ONE,
        y: Vector4::ONE,
        z: Vector4::ONE,
        w: Vector4::ONE
    };

    pub const EPSILON: Matrix4 = Matrix4 {
        x: Vector4::EPSILON,
        y: Vector4::EPSILON,
        z: Vector4::EPSILON,
        w: Vector4::EPSILON
    };

    pub fn new(
        r0c0: f32, r0c1: f32, r0c2: f32, r0c3: f32,
        r1c0: f32, r1c1: f32, r1c2: f32, r1c3: f32,
        r2c0: f32, r2c1: f32, r2c2: f32, r2c3: f32,
        r3c0: f32, r3c1: f32, r3c2: f32, r3c3: f32,
    ) -> Matrix4 {
        Matrix4 {
            x: Vector4::new(r0c0, r0c1, r0c2, r0c3),
            y: Vector4::new(r1c0, r1c1, r1c2, r1c3),
            z: Vector4::new(r2c0, r2c1, r2c2, r2c3),
            w: Vector4::new(r3c0, r3c1, r3c2, r3c3)
        }
    }

    pub fn c0(&self) -> Vector4 {
        [self.x.x, self.y.x, self.z.x, self.w.x].into()
    }

    pub fn c1(&self) -> Vector4 {
        [self.x.y, self.y.y, self.z.y, self.w.y].into()
    }

    pub fn c2(&self) -> Vector4 {
        [self.x.z, self.y.z, self.z.z, self.w.z].into()
    }

    pub fn c3(&self) -> Vector4 {
        [self.x.w, self.y.w, self.z.w, self.w.w].into()
    }

    pub fn from_rows(x: impl Into<Vector4>, y: impl Into<Vector4>, z: impl Into<Vector4>, w: impl Into<Vector4>) -> Matrix4 {
        Matrix4 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into()
        }
    }

    /// creates a perspective matrix for the specified settings, based on the opengl implementation
    pub fn persective_matrix(fovy: f32, aspect: f32, znear: f32, zfar: f32) -> Matrix4 {
        let f = 1.0 / (fovy / 2.0).tan();
        Matrix4::new(
            f / aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (znear + zfar ) / (znear - zfar), -1.0,
            0.0, 0.0, (2.0 * zfar * znear) / (znear - zfar), 0.0
        )
    }

    pub fn transpose(&mut self) {
        *self = self.transposed();
    }

    pub fn transposed(&self) -> Matrix4 {
        Matrix4::new(
            self.x.x, self.y.x, self.z.x, self.w.x,
            self.x.y, self.y.y, self.z.y, self.w.y,
            self.x.z, self.y.z, self.z.z, self.w.z,
            self.x.w, self.y.w, self.z.w, self.w.w
        )
    }

}

impl Into<[[f32; 4]; 4]> for Matrix4 {
    fn into(self) -> [[f32; 4]; 4] {
        [self.x.into(), self.y.into(), self.z.into(), self.w.into()]
    }
}

impl From<Matrix3> for Matrix4 {
    fn from(mat: Matrix3) -> Self {
        Matrix4::new(
            mat.x.x, mat.x.y, mat.x.z, 0.0,
            mat.y.x, mat.y.y, mat.y.z, 0.0,
            mat.z.x, mat.z.y, mat.z.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl From<Matrix2> for Matrix4 {
    fn from(value: Matrix2) -> Self {
        Matrix3::from(value).into()
    }
}


impl Mul for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4::new(
            self.x.dot(rhs.c0()), self.x.dot(rhs.c1()), self.x.dot(rhs.c2()), self.x.dot(rhs.c3()),
            self.y.dot(rhs.c0()), self.y.dot(rhs.c1()), self.y.dot(rhs.c2()), self.y.dot(rhs.c3()),
            self.z.dot(rhs.c0()), self.z.dot(rhs.c1()), self.z.dot(rhs.c2()), self.z.dot(rhs.c3()),
            self.w.dot(rhs.c0()), self.w.dot(rhs.c1()), self.w.dot(rhs.c2()), self.w.dot(rhs.c3())
        )
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: f32) -> Self::Output {
        Matrix4::from_rows(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs
        )
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4::new(
            self.x.dot(rhs),
            self.y.dot(rhs),
            self.z.dot(rhs),
            self.w.dot(rhs)
        )
    }
}