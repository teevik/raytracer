use crate::{
    bvh::Aabb,
    data::{Face, RayHit},
    extensions::RayExt,
    materials::Material,
};
use std::ops::Range;
use vek::{Ray, Vec3};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn get_aabb(self) -> Aabb {
        let radius = Vec3::broadcast(self.radius);

        Aabb::from_extremes(self.center - radius, self.center + radius)
    }

    pub fn raycast(&self, ray: Ray<f32>, range: Range<f32>) -> Option<RayHit> {
        let center_to_origin = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = Vec3::dot(center_to_origin, ray.direction);
        let c = center_to_origin.magnitude_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - discriminant_sqrt) / a;
        if !range.contains(&root) {
            root = (-half_b + discriminant_sqrt) / a;

            if !range.contains(&root) {
                return None;
            }
        }

        let distance = root;
        let point = ray.at(distance);

        let outward_normal = (point - self.center) / self.radius;
        let face = ray.get_face(outward_normal);

        let normal = match face {
            Face::Front => outward_normal,
            Face::Back => -outward_normal,
        };

        let material = self.material;

        Some(RayHit {
            distance,
            point,
            face,
            normal,
            material,
        })
    }
}
