use crate::data::Ray;
use crate::{
    bvh::Aabb,
    data::{Face, Hittable, RayHit},
    interval::Interval,
    materials::Material,
};
use std::f32::consts::PI;
use vek::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,

    pub bounding_box: Aabb,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32, material: Material) -> Self {
        let size = Vec3::broadcast(radius);
        let bounding_box = Aabb::from_extremes(center - size, center + size);

        Self {
            center,
            radius,
            bounding_box,
            material,
        }
    }
}

fn calculate_sphere_uv(point: Vec3<f32>) -> Vec2<f32> {
    let theta = f32::acos(-point.y);
    let phi = f32::atan2(-point.z, point.x) + PI;

    let u = phi / (2. * PI);
    let v = theta / PI;

    Vec2::new(u, v)
}

impl Hittable for Sphere {
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }

    fn raycast(&self, ray: Ray, interval: Interval) -> Option<RayHit> {
        let center_to_origin = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = Vec3::dot(center_to_origin, ray.direction);
        let c = center_to_origin.magnitude_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable interval
        let mut root = (-half_b - discriminant_sqrt) / a;
        if !interval.contains(root) {
            root = (-half_b + discriminant_sqrt) / a;

            if !interval.contains(root) {
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

        let uv = calculate_sphere_uv(outward_normal);

        let material = self.material.clone();

        Some(RayHit {
            distance,
            point,
            face,
            normal,
            uv,
            material,
        })
    }
}
