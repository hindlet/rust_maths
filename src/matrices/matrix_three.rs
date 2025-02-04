#![allow(dead_code)]
use super::super::vectors::Vector3;
use super::*;
use std::{ops::*, f32::consts::PI, fmt::Display};

#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Matrix3 {
    pub x: Vector3, 
    pub y: Vector3,
    pub z: Vector3,
}

impl Matrix3 {
    pub const IDENTITY: Matrix3 = Matrix3 {
        x: Vector3::X,
        y: Vector3::Y,
        z: Vector3::Z
    };

    pub const ONE: Matrix3 = Matrix3 {
        x: Vector3::ONE,
        y: Vector3::ONE,
        z: Vector3::ONE
    };

    pub const EPSILON: Matrix3 = Matrix3 {
        x: Vector3::EPSILON,
        y: Vector3::EPSILON,
        z: Vector3::EPSILON,
    };

    pub fn new(
        r0c0: f32, r0c1: f32 , r0c2: f32,
        r1c0: f32, r1c1: f32 , r1c2: f32,
        r2c0: f32, r2c1: f32 , r2c2: f32,
    ) -> Self {
        Matrix3 {
            x: Vector3::new(r0c0, r0c1, r0c2),
            y: Vector3::new(r1c0, r1c1, r1c2),
            z: Vector3::new(r2c0, r2c1, r2c2),
        }
    }

    pub fn from_rows(
        r0: impl Into<Vector3>,
        r1: impl Into<Vector3>,
        r2: impl Into<Vector3>,
    ) -> Self {
        Matrix3 {
            x: r0.into(),
            y: r1.into(),
            z: r2.into()
        }
    }

    pub fn from_columns(
        c0: impl Into<Vector3>,
        c1: impl Into<Vector3>,
        c2: impl Into<Vector3>,
    ) -> Self {
        let mat = Matrix3::from_rows(c0, c1, c2);
        mat.transposed()
    }

    pub fn c0(&self) -> Vector3 {
        Vector3::new(self.x.x, self.y.x, self.z.x)
    }

    pub fn c1(&self) -> Vector3 {
        Vector3::new(self.x.y, self.y.y, self.z.y)
    }

    pub fn c2(&self) -> Vector3 {
        Vector3::new(self.x.z, self.y.z, self.z.z)
    }


    /// creates a rotation maxtrix for anticlockwise angle around y axis
    pub fn from_angle_y(angle: f32) -> Self{
        Matrix3::new(
            angle.cos(), 0.0, angle.sin(),
            0.0, 1.0, 0.0,
            -angle.sin(), 0.0, angle.cos()
        )
    }

    /// creates a rotation maxtrix for anticlockwise angle around x axis
    pub fn from_angle_x(angle: f32) -> Self {
        Matrix3::new(
            1.0, 0.0, 0.0,
            0.0, angle.cos(), -angle.sin(),
            0.0, angle.sin(), angle.cos()
        )
    }

    /// creates a rotation maxtrix for anticlockwise angle around z axis
    pub fn from_angle_z(angle: f32) -> Self {
        Matrix3::new(
            angle.cos(), -angle.sin(), 0.0,
            angle.sin(), angle.cos(), 0.0,
            0.0, 0.0, 1.0
        )
    }

    /// creates a rotation matrix for anticlockwise rotation of angle around the specified axis
    pub fn from_angle_and_axis(angle: f32, axis: impl Into<Vector3>) -> Self {
        let mut axis: Vector3 = axis.into();
        axis.normalise();
        if angle == 0.0 {return Matrix3::IDENTITY;}
        Matrix3::new(
            angle.cos() + axis.x.powi(2) * (1.0 - angle.cos()),
            axis.x * axis.y * (1.0 - angle.cos()) - axis.z * angle.sin(),
            axis.x * axis.z * (1.0 - angle.cos()) + axis.y * angle.sin(),

            axis.y * axis.x * (1.0 - angle.cos()) + axis.z * angle.sin(),
            angle.cos() + axis.y.powi(2) * (1.0 - angle.cos()),
            axis.y * axis.z * (1.0 - angle.cos()) - axis.x * angle.sin(),

            axis.z * axis.x * (1.0 - angle.cos()) - axis.y * angle.sin(),
            axis.z * axis.y * (1.0 - angle.cos()) + axis.x * angle.sin(),
            angle.cos() + axis.z.powi(2) * (1.0 - angle.cos())
        )
    }

    pub fn from_euler_angles(angles: impl Into<Vector3>) -> Matrix3 {
        let angles = angles.into();
        let x = angles.x;
        let y = angles.y;
        let z = angles.z;
        Matrix3::new(
            y.cos() * z.cos(),
            x.sin() * y.sin() * z.cos() - x.cos() * z.sin(),
            x.cos() * y.sin() * z.cos() + x.sin() * z.sin(),

            y.cos() * z.sin(),
            x.sin() * y.sin() * z.sin() + x.cos() * z.cos(),
            x.cos() * y.sin() * z.sin() - x.sin() * z.cos(),

            -(y.sin()),
            x.sin() * y.cos(),
            x.cos() * y.cos()
        )
    }

    // calculates the euler angles required to create a specific matrix
    pub fn euler_angles_from(rot: Matrix3) -> Vector3 {
        let mut angles = Vector3::ZERO;

        // special cases
        if rot.z.x == 1.0{
            angles.y = -PI / 2.0;
            angles.x = -(rot.x.y).atan2(-rot.x.z);
            return angles;
        }
        if rot.z.x == -1.0 {
            angles.y = PI / 2.0;
            angles.x = rot.x.y.atan2(rot.x.z);
            return angles;
        }

        // get y angle
        angles.y = -rot.z.x.asin();

        // get x angle
        angles.x = (rot.z.y / angles.y.cos()).atan2(rot.z.z / angles.y.cos());

        // get z angle
        angles.z = (rot.y.x / angles.y.cos()).atan2(rot.x.x / angles.y.cos());

        angles
    }   


    // creates a transform matrix for scaling by specied multiplier
    pub fn from_scale(scale: f32) -> Self{
        Matrix3::new(
            scale, 0.0, 0.0,
            0.0, scale, 0.0,
            0.0, 0.0, scale
        )
    }

    pub fn transposed(&self) -> Matrix3{
        Matrix3::new(
            self.x.x, self.y.x, self.z.x,
            self.x.y, self.y.y, self.z.y,
            self.x.z, self.y.z, self.z.z
        )
    }

    // returns the determinant of a given matrix
    pub fn determinant(&self) -> f32 {
        self.x.x * (self.y.y * self.z.z - self.z.y * self.y.z)
            - self.x.y * (self.y.x * self.z.z - self.z.x * self.y.z)
            + self.x.z * (self.y.x * self.z.y - self.z.x * self.y.y)
    }

    // returns the inverse the given matrix, equivelent to matrix^-1
    pub fn inverted(&self) -> Matrix3{
        let det = self.determinant();
        if det == 0.0 {return self.clone();}

        let c0  = self.y.cross(self.z);
        let c1  = self.z.cross(self.x);
        let c2  = self.x.cross(self.y);
        Matrix3::from_columns(c0, c1, c2) / det
    }

    pub fn extend(&self) -> Matrix4 {
        Matrix4::new(
            self.x.x, self.x.y, self.x.z, 0.0,
            self.y.x, self.y.y, self.y.z, 0.0,
            self.z.x, self.z.y, self.z.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn truncate(&self) -> Matrix2 {
        Matrix2::new(
            self.x.x, self.x.y,
            self.y.x, self.y.y,
        )
    }
}



impl Mul for Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: Self) -> Self::Output {

        Matrix3::new(
            self.x.dot(rhs.c0()), self.x.dot(rhs.c1()), self.x.dot(rhs.c2()),
            self.y.dot(rhs.c0()), self.y.dot(rhs.c1()), self.y.dot(rhs.c2()),
            self.z.dot(rhs.c0()), self.z.dot(rhs.c1()), self.z.dot(rhs.c2()),
        )
    }
}

impl Add for Matrix3 {
    type Output = Matrix3;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix3::from_rows(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl Sub for Matrix3 {
    type Output = Matrix3;
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix3::from_rows(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(
            rhs.dot(self.x),
            rhs.dot(self.y),
            rhs.dot(self.z)
        )
    }
}

impl Mul<f32> for Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: f32) -> Self::Output {
        Matrix3::from_rows(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        )
    }
}

impl Div<f32> for Matrix3 {
    type Output = Matrix3;
    fn div(self, rhs: f32) -> Self::Output {
        Matrix3::from_rows(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        )
    }
}


impl Display for Matrix3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[\n   {},\n   {},\n   {}\n]", self.x, self.y, self.z)
    }
}