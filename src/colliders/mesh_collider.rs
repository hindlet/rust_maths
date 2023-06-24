#![allow(dead_code, unused_variables, unused_imports)]
use super::{Vector3, Collider, RayHitInfo, triangle_collider::TriangleCollider, AABoundingBox, quicksort};


#[derive(Default, Debug, PartialEq, Clone)]
pub struct MeshCollider {
    tris: Vec<TriangleCollider>,
    bounds: AABoundingBox
}


impl MeshCollider {
    pub fn new(vertices: Vec<Vector3>, indices: Vec<u32>) -> Self{
        let mut tris = Vec::new();

        for i in (0..indices.len()).step_by(3) {
            tris.push(TriangleCollider::new(vertices[indices[i] as usize], vertices[indices[i + 1] as usize], vertices[indices[i + 2] as usize]));
        }

        let bounds = AABoundingBox::from_points(vertices);
        MeshCollider {
            tris,
            bounds
        }
    }
}

impl Collider for MeshCollider {
    /// for checking intersections of a mesh and ray, I chose to sort the mesh triangles by distance from the ray root and loop through them
    /// 
    /// A mesh does not check if a point is contained as there is not guarantee there is an inside to a mesh
    fn check_ray(
        &self,
        root_position: impl Into<Vector3>,
        direction: impl Into<Vector3>,
        max_distance: Option<f32>,
    ) -> Option<RayHitInfo> {
        let (root_position, direction): (Vector3, Vector3) = (root_position.into(), direction.into());
        let direction = direction.normalised();

        if self.bounds.check_ray(root_position, direction, max_distance).is_none() {
            // println!("failed mesh collide on bounds\n- Origin: {:?} \n- Direction: {:?} \n- Bounds: {:?}", root_position, direction, self.bounds);
            return None;
        }

        // sort the tris by distance
        let dist_indices = {
            let mut indices = Vec::new();
            for i in 0..self.tris.len() {
                indices.push((self.tris[i].centre_dist_to(root_position), i));
            }
            indices
        };

        let sorted_indices = quicksort(dist_indices);
        for index in sorted_indices {
            let check = self.tris[index.1].check_ray(root_position, direction, max_distance);
            if check.is_some() {return check;}
        }
        None
    }
}

