use rust_maths::*;


///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// AABB ///////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod aabb_collider_tests {
    use super::{Vector3, AABoundingBox, Collider};
    #[test]
    fn perpendicular_intersection_test() {
        let bounds = AABoundingBox::new(Vector3::ZERO(), Vector3::ONE() * 5.0);
        let hit = bounds.check_ray([10, 2, 0], -Vector3::X(), Some(25.0)).unwrap();
        assert_eq!(hit.hit_position, [5, 2, 0].into());
        assert_eq!(hit.hit_distance, 5.0);
    }

    #[test]
    fn contained_intersection_test() {
        let bounds = AABoundingBox::new(Vector3::ZERO(), Vector3::ONE() * 5.0);
        let hit = bounds.check_ray([3, 2, 0], -Vector3::X(), Some(25.0)).unwrap();
        assert_eq!(hit.hit_position, [3, 2, 0].into());
        assert_eq!(hit.hit_distance, 0.0);
    }

    #[test]
    fn angled_test() {
        let bounds = AABoundingBox::new(Vector3::ZERO(), Vector3::ONE() * 5.0);
        let hit = bounds.check_ray([8, 2, 0], [-1, 0, 1], Some(25.0)).unwrap();
        assert_eq!(hit.hit_position, [5.0, 2.0, 2.9999998].into()); // this should be (5, 2, 3) but due to rounding errors from floats the last number is lightly off, within acceptable range
        assert_eq!(hit.hit_distance, 3.0 * 2_f32.sqrt());
    }

    #[test]
    fn zero_height_test() {
        let bounds = AABoundingBox::new([-10, 0, -10], [10, 0, 10]);
        let hit = bounds.check_ray([0, 10, 0], [0, -1, 0], Some(25.0)).unwrap();
        assert_eq!(hit.hit_position, [0, 0, 0].into()); // this should be (5, 2, 3) but due to rounding errors from floats the last number is lightly off, within acceptable range
        assert_eq!(hit.hit_distance, 10.0);
    }
}

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// sphere /////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod sphere_collider_tests {
    use super::{BoundingSphere, Collider};
    #[test]
    fn contained_point_test() {
        let sphere = BoundingSphere::new([0, 0, 0], 5.0);
        let hit = sphere.check_ray([0.0, 2.5, 0.0], [1, 0, 0], Some(2.0)).unwrap();
        assert_eq!(hit.hit_position, [0.0, 2.5, 0.0].into());
        assert_eq!(hit.hit_distance, 0.0);
    }

    #[test]
    fn intersection_test() {
        let sphere = BoundingSphere::new([0, 0, 0], 2.5);
        let hit = sphere.check_ray([5.0, 0.0, 0.0], [-1, 0, 0], Some(20.0)).unwrap();
        assert_eq!(hit.hit_position, [2.5, 0.0, 0.0].into());
        assert_eq!(hit.hit_distance, 2.5);
    }

    #[test]
    fn too_far_test() {
        let sphere = BoundingSphere::new([0, 0, 0], 2.5);
        let hit = sphere.check_ray([5.0, 0.0, 0.0], [-1, 0, 0], Some(2.0));
        assert!(hit.is_none())
    }

    #[test]
    fn no_intersection_test() {
        let sphere = BoundingSphere::new([0, 0, 0], 2.5);
        let hit = sphere.check_ray([5.0, 0.0, 0.0], [0, 1, 0], Some(25.0));
        assert!(hit.is_none())
    }
}


///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// Plane //////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod plane_collider_tests {
    use super::{PlaneCollider, Collider};

    #[test]
    fn parralel_ray_test() {
        let plane = PlaneCollider::new([0, 0, 0], [1, 1]);
        assert!(plane.check_ray([0, 1, 0], [1, 0, 0], Some(25.0)).is_none())
    }

    #[test]
    fn position_ray_test() {
        let plane = PlaneCollider::new([0, 0, 0], [1, 1]);
        let hit = plane.check_ray([0, 0, 0], [1, 0, 0], Some(25.0)).unwrap();
        assert_eq!(hit.hit_position, [0, 0, 0].into());
        assert_eq!(hit.hit_distance, 0.0);
    }

    #[test]
    fn contained_ray_test() {
        let plane = PlaneCollider::new([0, 0, 0], [5, 5]);
        let hit = plane.check_ray([1, 0, 1], [1, 0, 0], Some(25.0)).unwrap();
        assert_eq!(hit.hit_position, [1, 0, 1].into());
        assert_eq!(hit.hit_distance, 0.0);
    }

    #[test]
    fn ray_hit_test() {
        let plane = PlaneCollider::new([0, 0, 0], [5, 5]);
        let hit = plane.check_ray([0, 5, 0], [0, -1, 0], Some(25.0)).unwrap();
        assert_eq!(hit.hit_position, [0, 0, 0].into());
        assert_eq!(hit.hit_distance, 5.0);
    }
}

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// Mesh ///////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod mesh_collider_tests {
    use super::{MeshCollider, Collider};

    #[test]
    fn miss_mesh_test() {
        let mesh = MeshCollider::new(
            vec![[-2, 0, -2].into(), [-2, 0, 2].into(), [2, 0, -2].into(), [2, 0, 2].into(), [0, 5, 0].into()],
            vec![0, 3, 2, 3, 1, 0, 0, 4, 1, 1, 4, 2, 2, 4, 3, 3, 4, 1]
        );

        assert!(mesh.check_ray([1, 4, 0], [1, 0, 0], Some(25.0)).is_none())
    }

    #[test]
    fn miss_box_test() {
        let mesh = MeshCollider::new(
            vec![[-2, 0, -2].into(), [-2, 0, 2].into(), [2, 0, -2].into(), [2, 0, 2].into(), [0, 5, 0].into()],
            vec![0, 3, 2, 3, 1, 0, 0, 4, 1, 1, 4, 2, 2, 4, 3, 3, 4, 1]
        );

        assert!(mesh.check_ray([1, 6, 0], [1, 0, 0], Some(25.0)).is_none())
    }

    #[test]
    fn hit_test() {
        let mesh = MeshCollider::new(
            vec![[-2, 0, -2].into(), [-2, 0, 2].into(), [2, 0, -2].into(), [2, 0, 2].into(), [-2, 5, 0].into()],
            vec![0, 3, 2, 3, 1, 0, 0, 4, 1, 1, 4, 2, 2, 4, 3, 3, 4, 1]
        );

        let hit = mesh.check_ray([-5, 3, 0], [1, 0, 0], Some(25.0)).unwrap();

        assert_eq!(hit.hit_position, [-2, 3, 0].into());
        assert_eq!(hit.hit_distance, 3.0);
    }

    #[test]
    fn as_plane_test() {
        let collider = MeshCollider::new(
            vec![
            [-25.0, 0.0, -25.0].into(),
            [-25.0, 0.0, 25.0].into(),
            [25.0, 0.0, -25.0].into(),
            [25.0, 0.0, 25.0].into()],
            vec![0, 3, 2, 3, 0, 1]
        );

        let hit = collider.check_ray([5, 10, 5], [0, -1, 0], None).unwrap();

        assert_eq!(hit.hit_position, [5, 0, 5].into());
        assert_eq!(hit.hit_distance, 10.0);
    }

    #[test]
    fn slant_test() {
        let mesh = MeshCollider::new(
            vec![[-1, 0, -1].into(), [0, 0, -1].into(), [1, 0, -1].into(), [-1, 0, 0].into(), [0, 0, 0].into(), [1, 0, 0].into(), [-1, 0, 1].into(), [0, 0, 1].into(), [1, 0, 1].into()],
            vec![
                8, 7, 5, 4, 5, 7,
                7, 6, 4, 3, 4, 6,
                5, 4, 2, 1, 2, 4,
                4, 3, 1, 0, 1, 3
            ]
        );

        let hit = mesh.check_ray([-0.5, 5.0, -0.5], [0, -1, 0], None).unwrap();

        assert_eq!(hit.hit_position, [-0.5, 0.0, -0.5].into());
    }
}