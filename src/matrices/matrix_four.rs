#![allow(dead_code)]
use super::super::vectors::Vector4;
use super::*;
use std::{ops::*, fmt::Display};

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

    pub fn truncate(&self) -> Matrix3 {
        Matrix3::new(
            self.x.x, self.x.y, self.x.z,
            self.y.x, self.y.y, self.y.z,
            self.z.x, self.z.y, self.z.z
        )
    }

    pub fn determinant(&self) -> f32 {
        // self.x.x * (
        //     self.y.y * (self.z.z * self.w.w - self.w.z * self.z.w)
        //     - self.y.z * (self.z.y * self.w.w - self.w.y * self.z.w)
        //     + self.y.w * (self.z.y * self.w.z - self.w.y * self.z.z)
        // )
        // - self.x.y * (
        //     self.y.x * (self.z.z * self.w.w - self.w.z * self.z.w)
        //     - self.y.z * (self.z.x * self.w.w - self.w.x * self.z.w)
        //     + self.y.w * (self.z.x * self.w.z - self.w.z * self.z.z)
        // )
        // + self.x.z * (
        //     self.y.z * (self.z.y * self.w.w - self.w.y * self.z.w)
        //     - self.y.y * (self.z.x * self.w.w - self.w.z * self.z.w)
        //     + self.y.w * (self.z.x * self.w.y - self.w.x * self.z.y)
        // )
        // - self.x.w * (
        //     self.y.x * (self.z.y * self.w.z - self.w.y * self.z.z)
        //     - self.y.y * (self.z.x * self.w.z - self.w.x * self.z.z)
        //     + self.y.z * (self.z.x * self.w.y - self.w.x * self.z.y)
        // )
        self.x.x * Matrix3::from_rows(self.y.truncate_n(0), self.z.truncate_n(0), self.w.truncate_n(0)).determinant()
        - self.x.y * Matrix3::from_rows(self.y.truncate_n(1), self.z.truncate_n(1), self.w.truncate_n(1)).determinant()
        + self.x.z * Matrix3::from_rows(self.y.truncate_n(2), self.z.truncate_n(2), self.w.truncate_n(2)).determinant()
        - self.x.w * Matrix3::from_rows(self.y.truncate_n(3), self.z.truncate_n(3), self.w.truncate_n(3)).determinant()
    }

    // pain
    pub fn inverted(&self) -> Matrix4 {
        // let inv_det = 1.0 / self.determinant();

        // // -1^(1 + 1) = 1
        // let xx = self.y.y * (self.z.z * self.w.w - self.w.z * self.z.w)
        //     - self.y.z * (self.z.y * self.w.w - self.w.y * self.z.w)
        //     + self.y.w * (self.z.y * self.w.z - self.w.y * self.z.z);
        // // -1 & (1 + 2) = -1
        // let xy = -1.0 * self.y.x * (self.z.z * self.w.w - self.w.z * self.z.w)
        //     - self.y.z * (self.z.x * self.w.w - self.w.x * self.z.w)
        //     + self.y.w * (self.z.x * self.w.z - self.w.x * self.z.z);
        // // -1 ^ (1 + 3) = 1
        // let xz = self.y.x * (self.z.y * self.w.w - self.w.y * self.z.w)
        //     - self.y.y * (self.z.x * self.w.w - self.w.x * self.z.w)
        //     + self.y.w * (self.z.x * self.w.y - self.w.x * self.z.y);
        // // -1 & (1 + 4) = -1
        // let xw = -1.0 * self.y.x * (self.z.y * self.w.z - self.w.y * self.z.z)
        //     - self.y.y * (self.z.x * self.w.z - self.w.x * self.z.z)
        //     + self.y.z * (self.z.x * self.w.y - self.w.x * self.z.y);

        // // -1^(2 + 1) = -1
        // let yx = -1.0 * self.x.y * (self.z.z * self.w.w - self.w.z * self.z.w)
        //     - self.x.z * (self.z.y * self.w.w - self.w.y * self.z.w)
        //     + self.x.w * (self.z.y * self.w.z - self.w.y * self.z.z);
        // // -1 & (2 + 2) = 1
        // let yy = self.x.x * (self.z.z * self.w.w - self.w.z * self.z.w)
        //     - self.x.z * (self.z.x * self.w.w - self.w.x * self.z.w)
        //     + self.x.w * (self.z.x * self.w.z - self.w.x * self.z.z);
        // // -1 ^ (2 + 3) = -1
        // let yz = -1.0 * self.x.x * (self.z.y * self.w.w - self.w.y * self.z.w)
        //     - self.x.y * (self.z.x * self.w.w - self.w.x * self.z.w)
        //     + self.x.w * (self.z.x * self.w.y - self.w.x * self.z.y);
        // // -1 & (2 + 4) = 1
        // let yw = self.x.x * (self.z.y * self.w.z - self.w.y * self.z.z)
        //     - self.x.y * (self.z.x * self.w.z - self.w.x * self.z.z)
        //     + self.x.z * (self.z.x * self.w.y - self.w.x * self.z.y);

        // // -1^(3 + 1) = 1
        // let zx = self.x.y * (self.y.z * self.w.w - self.w.z * self.y.w)
        //     - self.x.z * (self.y.y * self.w.w - self.w.y * self.y.w)
        //     + self.x.w * (self.y.y * self.w.z - self.w.y * self.y.z);
        // // -1 & (3 + 2) = -1
        // let zy = -1.0 * self.x.x * (self.y.z * self.w.w - self.w.z * self.y.w)
        //     - self.x.z * (self.y.x * self.w.w - self.w.x * self.y.w)
        //     + self.x.w * (self.y.x * self.w.z - self.w.x * self.y.z);
        // // -1 ^ (3 + 3) = 1
        // let zz = self.x.x * (self.y.y * self.w.w - self.w.y * self.y.w)
        //     - self.x.y * (self.y.x * self.w.w - self.w.x * self.y.w)
        //     + self.x.w * (self.y.x * self.w.y - self.w.x * self.y.y);
        // // -1 & (3 + 4) = -1
        // let zw = -1.0 * self.x.x * (self.y.y * self.w.z - self.w.y * self.y.z)
        //     - self.x.y * (self.y.x * self.w.z - self.w.x * self.y.z)
        //     + self.x.z * (self.y.x * self.w.y - self.w.x * self.y.y);

        // // -1^(4 + 1) = -1
        // let wx = -1.0 * self.x.y * (self.y.z * self.z.w - self.z.z * self.y.w)
        //     - self.x.z * (self.y.y * self.z.w - self.z.y * self.y.w)
        //     + self.x.w * (self.y.y * self.z.z - self.z.y * self.y.z);
        // // -1 & (4 + 2) = 1
        // let wy = self.x.x * (self.y.z * self.z.w - self.z.z * self.y.w)
        //     - self.x.z * (self.y.x * self.z.w - self.z.x * self.y.w)
        //     + self.x.w * (self.y.x * self.z.z - self.z.x * self.y.z);
        // // -1 ^ (4 + 3) = -1
        // let wz = -1.0 * self.x.x * (self.y.y * self.z.w - self.z.y * self.y.w)
        //     - self.x.y * (self.y.x * self.z.w - self.z.x * self.y.w)
        //     + self.x.w * (self.y.x * self.z.y - self.z.x * self.y.y);
        // // -1 & (4 + 4) = 1
        // let ww = self.x.x * (self.y.y * self.z.z - self.z.y * self.y.z)
        //     - self.x.y * (self.y.x * self.z.z - self.z.x * self.y.z)
        //     + self.x.z * (self.y.x * self.z.y - self.z.x * self.y.y);

        // Matrix4::new(
        //     xx, xy, xz, xw,
        //     yx, yy, yz, yw,
        //     zx, zy, zz, zw,
        //     wx, wy, wz, ww
        // ) * inv_det
        let det = self.determinant();
        if det == 0.0 {
            self.clone()
        } else {
            let inv_det = 1.0 / det;
            let t = self.transposed();
            let cf = |i, j| {
                let mat = match i {
                    0 => {
                        Matrix3::from_columns(t.y.truncate_n(j), t.z.truncate_n(j), t.w.truncate_n(j))
                    }
                    1 => {
                        Matrix3::from_columns(t.x.truncate_n(j), t.z.truncate_n(j), t.w.truncate_n(j))
                    }
                    2 => {
                        Matrix3::from_columns(t.x.truncate_n(j), t.y.truncate_n(j), t.w.truncate_n(j))
                    }
                    3 => {
                        Matrix3::from_columns(t.x.truncate_n(j), t.y.truncate_n(j), t.z.truncate_n(j))
                    }
                    _ => panic!("out of range"),
                };
                let sign = if (i + j) & 1 == 1 {
                    -1.0
                } else {
                    1.0
                };
                mat.determinant() * sign * inv_det
            };


            Matrix4::new(
                cf(0, 0), cf(0, 1), cf(0, 2), cf(0, 3),
                cf(1, 0), cf(1, 1), cf(1, 2), cf(1, 3),
                cf(2, 0), cf(2, 1), cf(2, 2), cf(2, 3),
                cf(3, 0), cf(3, 1), cf(3, 2), cf(3, 3),
            )
        }
    }

}

impl Into<[[f32; 4]; 4]> for Matrix4 {
    fn into(self) -> [[f32; 4]; 4] {
        [self.x.into(), self.y.into(), self.z.into(), self.w.into()]
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

impl Add for Matrix4 {
    type Output = Matrix4;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix4::from_rows(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w
        )
    }
}

impl Sub for Matrix4 {
    type Output = Matrix4;
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix4::from_rows(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w
        )
    }
}


impl Display for Matrix4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[\n   {},\n   {},\n   {},\n   {}\n]", self.x, self.y, self.z, self.w)
    }
}