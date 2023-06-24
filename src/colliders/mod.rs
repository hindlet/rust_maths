use super::{vectors::*, matrices::Matrix3, quicksort, solve_quadratic};

mod mesh_collider;
mod plane_collider;
mod triangle_collider;
mod aabb;
mod bounding_sphere;

pub use mesh_collider::MeshCollider;
pub use bounding_sphere::BoundingSphere;
pub use plane_collider::PlaneCollider;
pub use triangle_collider::TriangleCollider;
pub use aabb::AABoundingBox;


pub trait Collider {
    fn check_ray(
        &self,
        root_position: impl Into<Vector3>,
        direction: impl Into<Vector3>,
        max_distance: Option<f32>,
    ) -> Option<RayHitInfo>;
}

#[derive(Debug)]
pub struct RayHitInfo {
    pub hit_position: Vector3,
    pub hit_distance: f32,
    pub hit_normal: Vector3,
}

impl RayHitInfo {
    pub fn new(position: Vector3, dist: f32, surface_normal: Vector3) -> Self{
        RayHitInfo{
            hit_position: position,
            hit_distance: dist,
            hit_normal: surface_normal
        }
    }
}