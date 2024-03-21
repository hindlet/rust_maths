use rust_maths::*;
use std::f32::consts::PI;

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// Matrix 3 ///////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn euler_angles_x_test() {
        let angles = Vector3::new(PI, 0.0, 0.0);

        assert_eq!(Matrix3::from_angle_x(PI), Matrix3::from_euler_angles(angles))
    }

    #[test]
    fn euler_angles_y_test() {
        let angles = Vector3::new(0.0, PI, 0.0);

        assert_eq!(Matrix3::from_angle_y(PI), Matrix3::from_euler_angles(angles))
    }

    #[test]
    fn euler_angles_z_test() {
        let angles = Vector3::new(0.0, 0.0, PI);

        assert_eq!(Matrix3::from_angle_z(PI), Matrix3::from_euler_angles(angles))
    }

    #[test]
    fn euler_angles_all_test() {
        let from_angles = Matrix3::from_euler_angles([PI, PI, PI]);
        let from_multiply = Matrix3::from_angle_x(PI) * Matrix3::from_angle_y(PI) * Matrix3::from_angle_z(PI);

        assert!(from_multiply - Matrix3::EPSILON <= from_angles && from_multiply + Matrix3::EPSILON >= from_angles);
    }

    #[test]
    fn determinant_test() {
        let mat2 = Matrix2::new(
            2.0, 1.0,
            -1.0, 3.0
        );
        let mat3 = Matrix3::new(
            3.0, 4.0, 6.0,
            7.0, 8.0, 9.0,
            2.0, 1.0, 3.0
        );
        let mat4 = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            6.0, 7.0, 3.0, 4.0,
            3.0, 2.0, 7.0, 9.0,
            10.0, 11.0, 5.0, 4.0
        );

        assert_eq!(7.0, mat2.det());
        assert_eq!(-21.0, mat3.determinant());
        assert_eq!(130.0, mat4.determinant());
    }

    #[test]
    fn from_rows_test() {
        let r0 = Vector3::new(3.0, 4.0, 6.0);
        let r1 = Vector3::new(7.0, 8.0, 9.0);
        let r2 = Vector3::new(2.0, 1.0, 3.0);

        assert_eq!(
            Matrix3::from_rows(r0, r1, r2),
            Matrix3::new(
                3.0, 4.0, 6.0,
                7.0, 8.0, 9.0,
                2.0, 1.0, 3.0
            )
        )
    }

    #[test]
    fn from_columns_test() {
        let c0 = Vector3::new(3.0, 7.0, 2.0);
        let c1 = Vector3::new(4.0, 8.0, 1.0);
        let c2 = Vector3::new(6.0, 9.0, 3.0);

        assert_eq!(
            Matrix3::from_columns(c0, c1, c2),
            Matrix3::new(
                3.0, 4.0, 6.0,
                7.0, 8.0, 9.0,
                2.0, 1.0, 3.0
            )
        )
    }

    #[test]
    fn invert_tests() {
        let mat2 = Matrix2::new(
            2.0, 5.0,
            -4.0, 2.0
        );
        let resmat2 = mat2.inverted() * mat2;
        let mat3 = Matrix3::new(
            3.0, 4.0, 6.0,
            7.0, 8.0, 9.0,
            2.0, 1.0, 3.0
        );
        let resmat3 = mat3.inverted() * mat3;
        let mat4 = Matrix4::new(
            5.0, -2.0, 1.0, 23.0,
            0.0, 3.0, 4.5, 5.5,
            3.3, 4.5, 1.2, 0.7,
            4.5, 6.6, 7.4, 8.8
        );
        let resmat4 = mat4.inverted() * mat4;
        println!("{}", resmat4);

        assert!(Matrix2::IDENTITY - Matrix2::EPSILON <= resmat2 && Matrix2::IDENTITY + Matrix2::EPSILON >= resmat2);
        assert!(Matrix3::IDENTITY - Matrix3::EPSILON <= resmat3 && Matrix3::IDENTITY + Matrix3::EPSILON >= resmat3);
        assert!(Matrix4::IDENTITY - Matrix4::EPSILON <= resmat4 && Matrix4::IDENTITY + Matrix4::EPSILON >= resmat4)
    } 

    #[test]
    fn axis_angle_test() {
        let axis = Vector3::new(0.0, 0.0, 0.5_f32.sqrt());
        let angle = - PI / 4.0;

        assert_eq!(Matrix3::from_angle_and_axis(angle, axis),
        Matrix3::new(
            0.70710677, 0.70710677, 0.0,
            -0.70710677, 0.70710677, 0.0,
            0.0, 0.0, 1.0
        ))
    }

    #[test]
    fn identity_tests() {
        let vector2 = Vector2::new(15.0, 6.7);
        let vector3 = Vector3::new(56.7, 125.5, 197.2);
        let vector4 = Vector4::new(653.0, 56.8, 328.5, 674.6);

        assert_eq!(Matrix2::IDENTITY * vector2, vector2);
        assert_eq!(Matrix3::IDENTITY * vector3, vector3);
        assert_eq!(Matrix4::IDENTITY * vector4, vector4)
    }

}