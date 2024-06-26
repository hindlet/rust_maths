use rust_maths::*;


///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// Vector3 ////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod vector_three_tests {
    use super::{Matrix3, Vector3};

    #[test]
    fn transform_test() {
        let mat = Matrix3::new(
            0.7074, 0.7068, 0.0,
            -0.7068, 0.7074, 0.0,
            0.0, 0.0, 1.0
        );
        let point = Vector3::new(2.102, 1.65, 1.195);
        assert_eq!(mat * point, Vector3::new(2.6531748, -0.3184836, 1.195))
    }

    #[test]
    fn cross_test() {
        assert_eq!(Vector3::new(0.0, 0.0, 1.0).cross([1, 0 ,0]), Vector3::new(0.0, 1.0, 0.0))
    }

    #[test]
    fn ordering_tests() {
        assert!(Vector3::X * 5.0 > Vector3::X);

        assert!(Vector3::X > Vector3::Y);
        assert!(Vector3::Y > Vector3::Z);
        assert!(Vector3::X > Vector3::Y + Vector3::Z);
        assert_eq!(Vector3::X, Vector3::X);
        assert!(Vector3::X < Vector3::X + Vector3::Y);
        assert!(Vector3::X + Vector3::Y < Vector3::X + Vector3::Y + Vector3::Z);
    }


    #[test]
    fn multiplication_test() {
        assert_eq!(Vector3::new(5.0, 4.0, 3.0) * Vector3::new(3.0, 4.0, 5.0), Vector3::new(15.0, 16.0, 15.0))
    }
}