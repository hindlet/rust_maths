#![allow(dead_code, unused_variables, unused_imports)]
use super::{Vector3, bounding_sphere::BoundingSphere, Collider, RayHitInfo};


#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct AABoundingBox {
    pub min_corner: Vector3, // least x y and z
    pub max_corner: Vector3,
}


impl AABoundingBox {
    pub fn new(min_corner: impl Into<Vector3>, max_corner: impl Into<Vector3>) -> Self {
        AABoundingBox {
            min_corner: min_corner.into(),
            max_corner: max_corner.into()
        }
    }

    #[allow(non_snake_case)]
    pub fn ZERO() -> Self {
        AABoundingBox::new(Vector3::ZERO, Vector3::ZERO)
    }


    pub fn contains_point(&self, point: Vector3) -> bool {
        if (point.x < self.min_corner.x) || (point.x > self.max_corner.x) {
            return false
        }
        if (point.y < self.min_corner.y) || (point.y > self.max_corner.y) {
            return false
        }
        if (point.z < self.min_corner.z) || (point.z > self.max_corner.z) {
            return false
        }
        true
    }

    pub fn contains_points(&self, points: Vec<Vector3>) -> bool {
        for point in points {
            if !self.contains_point(point) {return false}
        }
        true
    }

    pub fn contains_sphere(&self, sphere: BoundingSphere) -> bool{
        if !self.contains_point(sphere.centre) {return false;}

        if (sphere.centre.x - sphere.radius) < self.min_corner.x {return false;}
        if (sphere.centre.x + sphere.radius) > self.max_corner.x {return false;}

        if (sphere.centre.y - sphere.radius) < self.min_corner.y {return false;}
        if (sphere.centre.y + sphere.radius) > self.max_corner.y {return false;}

        if (sphere.centre.z - sphere.radius) < self.min_corner.z {return false;}
        if (sphere.centre.z + sphere.radius) > self.max_corner.z {return false;}

        true
    }

    pub fn contains_spheres(&self, spheres: Vec<BoundingSphere>) -> bool {
        for sphere in spheres {
            if !self.contains_sphere(sphere) {return  false}
        }
        true
    }

    pub fn from_spheres(spheres: Vec<BoundingSphere>) -> AABoundingBox {
        if spheres.len() == 0 {
            return AABoundingBox::ZERO();
        }


        let mut x_min = spheres[0].centre.x - spheres[0].radius;
        let mut x_max = spheres[0].centre.x + spheres[0].radius;

        let mut y_min = spheres[0].centre.y - spheres[0].radius;
        let mut y_max = spheres[0].centre.y + spheres[0].radius;

        let mut z_min = spheres[0].centre.z - spheres[0].radius;
        let mut z_max = spheres[0].centre.z + spheres[0].radius;

        for sphere in spheres.iter() {
            let sphere_x_min = sphere.centre.x - sphere.radius;
            let sphere_x_max = sphere.centre.x + sphere.radius;

            let sphere_y_min = sphere.centre.y - sphere.radius;
            let sphere_y_max = sphere.centre.y + sphere.radius;

            let sphere_z_min = sphere.centre.z - sphere.radius;
            let sphere_z_max = sphere.centre.z + sphere.radius;

            if sphere_x_min < x_min {x_min = sphere_x_min}
            if sphere_x_max > x_max {x_max = sphere_x_max}
            
            if sphere_y_min < y_min {y_min = sphere_y_min}
            if sphere_y_max > y_max {y_max = sphere_y_max}

            if sphere_z_min < z_min {z_min = sphere_z_min}
            if sphere_z_max > z_max {z_max = sphere_z_max}
        }

        let min_corner = Vector3::new(x_min, y_min, z_min);
        let max_corner =  Vector3::new(x_max, y_max, z_max);

        AABoundingBox::new(min_corner, max_corner)
    }

    pub fn from_points(points: Vec<Vector3>) -> AABoundingBox{
        if points.len() == 0 {return AABoundingBox::ZERO()}

        let mut x_min = points[0].x;
        let mut x_max = points[0].x;
        let mut y_min = points[0].y;
        let mut y_max = points[0].y;
        let mut z_min = points[0].z;
        let mut z_max = points[0].z;
         

        for point in points {
            if point.x < x_min {x_min = point.x}
            if point.x > x_max {x_max = point.x}

            if point.y < y_min {y_min = point.y}
            if point.y > y_max {y_max = point.y}

            if point.z < z_min {z_min = point.z}
            if point.z > z_max {z_max = point.z}
        }

        let min_corner = Vector3::new(x_min, y_min, z_min);
        let max_corner = Vector3::new(x_max, y_max, z_max);

        AABoundingBox::new(min_corner, max_corner)
    }

    pub fn is_intersecting_box(&self, other: AABoundingBox) -> bool {
        self.min_corner.x <= other.max_corner.x &&
        self.max_corner.x >= other.min_corner.x &&
        self.min_corner.y <= other.max_corner.y &&
        self.max_corner.y >= other.min_corner.y &&
        self.min_corner.z <= other.max_corner.z &&
        self.max_corner.z >= other.min_corner.z 
    }

}



impl Collider for AABoundingBox {
    /// intersections with a box algorithm taken from Amy Williams et al. 2004
    fn check_ray(
        &self,
        root_position: impl Into<Vector3>,
        direction: impl Into<Vector3>,
        max_distance: Option<f32>,
    ) -> Option<RayHitInfo> {
        let (root_position, direction): (Vector3, Vector3) = (root_position.into(), direction.into());
        let direction = direction.normalised();

        if self.contains_point(root_position) {return Some(RayHitInfo::new(root_position, 0.0, Vector3::ZERO));}

        let inv_dir = Vector3::ONE / direction;
        let (mut max_norm, mut min_norm) = (Vector3::X, -Vector3::X);

        let (mut tmax, mut tmin) = if inv_dir.x < 0.0 {
            ((self.min_corner.x - root_position.x) * inv_dir.x, (self.max_corner.x - root_position.x) * inv_dir.x)
        } else {
            ((self.max_corner.x - root_position.x) * inv_dir.x, (self.min_corner.x - root_position.x) * inv_dir.x)
        };

        let (t_ymax, t_ymin) = if inv_dir.y < 0.0 {
            ((self.min_corner.y - root_position.y) * inv_dir.y, (self.max_corner.y - root_position.y) * inv_dir.y)
        } else {
            ((self.max_corner.y - root_position.y) * inv_dir.y, (self.min_corner.y - root_position.y) * inv_dir.y)
        };


        if (tmin > t_ymax) || (t_ymin > tmax) {return None;}

        if t_ymin > tmin {tmin = t_ymin; min_norm = -Vector3::Y}
        if t_ymax < tmax {tmax = t_ymax; max_norm = Vector3::Y}

        let (t_zmax, t_zmin) = if inv_dir.z < 0.0 {
            ((self.min_corner.z - root_position.z) * inv_dir.z, (self.max_corner.z - root_position.z) * inv_dir.z)
        } else {
            ((self.max_corner.z - root_position.z) * inv_dir.z, (self.min_corner.z - root_position.z) * inv_dir.z)
        };

        if (tmin > t_zmax) || (t_zmin > tmax) {return None;}

        if t_zmin > tmin {tmin = t_zmin; min_norm = -Vector3::Z}
        if t_zmax < tmax {tmax = t_zmax; max_norm = Vector3::Z}

        let dist = if tmin < 0.0 {tmax} else if tmax < 0.0 {return None} else {tmin};

        let (dist, out_norm) = if tmin < 0.0 {(tmax, max_norm)} else if tmax < 0.0 {return None} else {(tmin, min_norm)};

        if max_distance.is_some() && dist > max_distance.unwrap() {return None;}

        Some(RayHitInfo::new(root_position + direction * dist, dist, out_norm))
    }
}