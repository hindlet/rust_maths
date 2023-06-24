use rust_maths::*;
use std::f32::consts::PI;

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// Matrix 3 ///////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod matrix_tests {
    use super::{Vector3, Matrix3, PI};

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
        let angles = Vector3::new(PI, PI, PI);

        // these are functionally the same, but there is a tiny difference in rounding, so they will not be equal
        //
        // left: `Matrix3 { x: Vector3 { x: 1.0, y: -8.742278e-8, z: -8.742278e-8 }, y: Vector3 { x: 8.742277e-8, y: 1.0, z: -8.742278e-8 }, z: Vector3 { x: 8.7422784e-8, y: 8.742277e-8, z: 1.0 } }`,
        // right: `Matrix3 { x: Vector3 { x: 1.0, y: -8.7422784e-8, z: -8.742277e-8 }, y: Vector3 { x: 8.742278e-8, y: 1.0, z: -8.7422784e-8 }, z: Vector3 { x: 8.742278e-8, y: 8.742278e-8, z: 1.0 } }
        assert_ne!(Matrix3::from_angle_x(PI) * Matrix3::from_angle_y(PI) * Matrix3::from_angle_z(PI), Matrix3::from_euler_angles(angles))
    }

    #[test]
    fn determinant_test() {
        let mat = Matrix3::new(
            3.0, 4.0, 6.0,
            7.0, 8.0, 9.0,
            2.0, 1.0, 3.0
        );

        assert_eq!(-21.0, mat.determinant())
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
    fn invert_test() {
        let mat = Matrix3::new(
            3.0, 4.0, 6.0,
            7.0, 8.0, 9.0,
            2.0, 1.0, 3.0
        );

        assert_eq!(
            mat.inverted(),
            Matrix3::new(
                -5.0 / 7.0, 2.0 / 7.0, 4.0 / 7.0,
                1.0 / 7.0, 1.0 / 7.0, -5.0 / 7.0,
                3.0 / 7.0, -5.0 / 21.0, 4.0 / 21.0
            )
        )
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

}