use rust_maths::*;


///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// AABB ///////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod bounding_box_tests {
    use super::{Vector3, AABoundingBox, BoundingSphere};

    #[test]
    fn zero_points_test() {
        let points: Vec<Vector3> = vec![];

        assert_eq!(AABoundingBox::from_points(points), AABoundingBox::ZERO())
    }

    #[test]
    fn defined_bounds_point_test() {
        let points: Vec<Vector3> = vec![
            Vector3{x: 0.0, y: 0.0, z: 0.0},
            Vector3{x: 0.0, y: 2.5, z:0.0},
            Vector3{x: 5.0, y: 1.5, z: 1.5},
        ];
        let bounds = AABoundingBox::new(Vector3::ZERO(), Vector3::new(5.0, 2.5, 1.5));
        assert_eq!(bounds.contains_points(points), true)
    }

    #[test]
    fn calculated_bounds_point_test() {
        let points: Vec<Vector3> = vec![
            Vector3{x: 0.0, y: 0.0, z: 0.0},
            Vector3{x: 0.0, y: 2.5, z:0.0},
            Vector3{x: 5.0, y: 1.5, z: 1.5},
        ];
        let bounds = AABoundingBox::from_points(points.clone());
        assert_eq!(bounds.contains_points(points), true)
    }

    #[test]
    fn defined_bounds_sphere_test() {
        let spheres: Vec<BoundingSphere> = vec![
            BoundingSphere {
                centre: Vector3{x: 0.0, y: 0.0, z: 0.0},
                radius: 5.0,
            },
            BoundingSphere {
                centre: Vector3{x: 0.0, y: 25.0, z:0.0},
                radius: 7.0,
            },
            BoundingSphere {
                centre: Vector3{x: 5.0, y: 15.0, z: 15.0},
                radius: 2.0,
            },
        ];
        let bounds = AABoundingBox::new(Vector3::new(-7.0, -5.0, -7.0), Vector3::new(7.0, 32.0, 17.0));
        assert_eq!(bounds.contains_spheres(spheres), true)
    }

    #[test]
    fn calculated_bounds_sphere_test() {
        let spheres: Vec<BoundingSphere> = vec![
            BoundingSphere {
                centre: Vector3{x: 0.0, y: 0.0, z: 0.0},
                radius: 5.0,
            },
            BoundingSphere {
                centre: Vector3{x: 0.0, y: 25.0, z:0.0},
                radius: 7.0,
            },
            BoundingSphere {
                centre: Vector3{x: 5.0, y: 15.0, z: 15.0},
                radius: 2.0,
            },
        ];
        let bounds = AABoundingBox::from_spheres(spheres.clone());

        assert_eq!(bounds.contains_spheres(spheres), true)
    }

    #[test]
    fn intersection_test() {
        let box_one = AABoundingBox::new(Vector3::ZERO(), Vector3::ONE() * 5.0);
        let box_two = AABoundingBox::new(Vector3::ONE() * 2.5, Vector3::ONE() * 5.0);
        assert_eq!(box_one.is_intersecting_box(box_two), true)
    }

    #[test]
    fn non_intersection_test() {
        let box_one = AABoundingBox::new(Vector3::ZERO(), Vector3::ONE() * 2.0);
        let box_two = AABoundingBox::new(Vector3::ONE() * 2.5, Vector3::ONE() * 5.0);
        assert_eq!(box_one.is_intersecting_box(box_two), false)
    }

}

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// Bounding Sphere ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod bounding_sphere_tests {
    use super::{Vector3, BoundingSphere};
    use std::f32::consts::PI;

    #[test]
    fn axis_furthest_point_test() {
        let sphere = BoundingSphere{
            centre: Vector3 { x: 0.0, y: 5.0, z: 0.0 },
            radius: 2.5,
        };
        let point = Vector3 {x: 0.0, y: 0.0, z: 0.0};

        let furthest_point = sphere.furthest_point_from_point(point);

        assert_eq!(furthest_point, Vector3 {x: 0.0, y: 7.5, z: 0.0})
    }

    #[test]
    fn non_axis_furthest_point_test() {
        let sphere = BoundingSphere{
            centre: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            radius: 2.5,
        };
        let point = Vector3 {x: -2.5, y: 0.0, z: 0.0};

        let furthest_point = sphere.furthest_point_from_point(point);

        assert_eq!(furthest_point, Vector3 {x: 2.5, y: 0.0, z: 0.0})
    }

    #[test]
    fn zero_points_test() {
        let points: Vec<Vector3> = vec![];

        assert_eq!(BoundingSphere::from_points(points), BoundingSphere::ZERO())
    }

    #[test]
    fn defined_bounds_test() {
        let points: Vec<Vector3> = vec![
            Vector3{x: 0.0, y: 0.0, z: 0.0},
            Vector3{x: 0.0, y: 2.5, z:0.0},
            Vector3{x: 0.0, y: 1.5, z: 1.5},
        ];
        let mut bounds = BoundingSphere::ZERO();
        bounds.radius = 2.5;
        assert!(bounds.contains_points(points))
    }

    #[test]
    fn calculated_bounds_test() {
        let points: Vec<Vector3> = vec![
            Vector3{x: 0.0, y: 0.0, z: 0.0},
            Vector3{x: 0.0, y: 2.5, z:0.0},
            Vector3{x: 0.0, y: 1.5, z: 1.5},
        ];
        let bounds = BoundingSphere::from_points(points.clone());
        assert_eq!(bounds.contains_points(points), true)
    }

    #[test]
    fn intersection_test() {
        let sphere_one = BoundingSphere {
            centre: Vector3::ZERO(),
            radius: 5.0,
        };
        let sphere_two = BoundingSphere {
            centre: Vector3 {x: 5.0, y: 0.0, z: 0.0},
            radius: 2.0,
        };

        assert_eq!(sphere_one.is_intersecting_sphere(sphere_two), true)
    }

    #[test]
    fn touching_test() {
        let sphere_one = BoundingSphere {
            centre: Vector3::ZERO(),
            radius: 5.0,
        };
        let sphere_two = BoundingSphere {
            centre: Vector3 {x: 10.0, y: 0.0, z: 0.0},
            radius: 5.0,
        };

        assert_eq!(sphere_one.is_intersecting_sphere(sphere_two), false)
    }

    #[test]
    fn non_intersection_test() {
        let sphere_one = BoundingSphere {
            centre: Vector3::ZERO(),
            radius: 5.0,
        };
        let sphere_two = BoundingSphere {
            centre: Vector3 {x: 10.0, y: 0.0, z: 0.0},
            radius: 2.0,
        };

        assert_eq!(sphere_one.is_intersecting_sphere(sphere_two), false)
    }

    #[test]
    fn intersection_volume_test() {
        let sphere_one = BoundingSphere {
            centre: Vector3::ZERO(),
            radius: 2.0,
        };
        let sphere_two = BoundingSphere {
            centre: Vector3 {x: -2.0, y: 2.0, z: -1.0},
            radius: 2.0,
        };

        assert_eq!(sphere_one.get_intersection_volume(&sphere_two), PI * 11.0 / 12.0)
    }

}