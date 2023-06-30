use super::{Vector3, Vector2, Collider, RayHitInfo};


/// Axis Alligned Plane Collider
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct PlaneCollider {
    position: Vector3,
    x_length: f32,
    z_length: f32,
    centre: Vector3,
}

impl PlaneCollider {
    pub fn new(position: impl Into<Vector3>, size: impl Into<Vector2>) -> Self{
        let size = size.into();
        let position = position.into();
        let scale: Vector3 = [size.x / 2.0, 0.0, size.y / 2.0].into();
        PlaneCollider {
            position: position,
            x_length: size.x,
            z_length: size.y,
            centre: position + scale
        }
    }
}

impl Collider for PlaneCollider {
    fn check_ray(
        &self,
        root_position: impl Into<Vector3>,
        direction: impl Into<Vector3>,
        max_distance: Option<f32>,
    ) -> Option<RayHitInfo> {
        let (root_position, direction): (Vector3, Vector3) = (root_position.into(), direction.into());
        let direction = direction.normalised();

        if (root_position == self.position) || ((self.centre - root_position).dot(Vector3::Y) == 0.0) {
            return Some(RayHitInfo::new(root_position, 0.0, Vector3::Y));
        }

        if direction.dot(Vector3::Y) == 0.0 {return None;}

        let distance = (self.centre - root_position).dot(Vector3::Y) / direction.dot(Vector3::Y);

        if max_distance.is_some() && distance > max_distance.unwrap() {return None;}

        let point = root_position + direction * distance;

        if (point.x - self.position.x > self.x_length) || (point.z - self.position.z > self.z_length) {return None;}

        Some(RayHitInfo::new(point, distance, Vector3::Y))
    }
}

