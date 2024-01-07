use vek::{Vec2, Vec3};

use crate::{
    bvh::Aabb,
    data::{Face, Hittable, Ray, RayHit},
    interval::Interval,
    materials::Material,
};

#[derive(Debug, Clone)]
pub struct Quad {
    pub origin: Vec3<f32>,
    pub u: Vec3<f32>,
    pub v: Vec3<f32>,

    pub normal: Vec3<f32>,
    pub w: Vec3<f32>,
    pub distance: f32,

    pub bounding_box: Aabb,
    pub material: Material,
}

impl Quad {
    pub fn new(origin: Vec3<f32>, u: Vec3<f32>, v: Vec3<f32>, material: Material) -> Quad {
        let n = u.cross(v);
        let w = n / n.dot(n);

        let normal = n.normalized();
        let distance = normal.dot(origin);

        let bounding_box = Aabb::from_extremes(origin, origin + u + v).padded();

        Self {
            origin,
            u,
            v,

            normal,
            w,
            distance,

            bounding_box,
            material,
        }
    }
}

fn is_interior(alpha: f32, beta: f32) -> bool {
    let is_outerior = (alpha < 0.) || (1. < alpha) || (beta < 0.) || (1. < beta);

    !is_outerior
}

impl Hittable for Quad {
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }

    fn raycast(&self, ray: Ray, interval: Interval) -> Option<RayHit> {
        let outward_normal = self.normal;
        let denominator = self.normal.dot(ray.direction);

        if denominator.abs() < 1e-8 {
            return None;
        }

        let distance = (self.distance - self.normal.dot(ray.origin)) / denominator;

        if !interval.contains(distance) {
            return None;
        }

        let point = ray.at(distance);
        let planat_hit_vector = point - self.origin;
        let alpha = self.w.dot(planat_hit_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planat_hit_vector));

        if !is_interior(alpha, beta) {
            return None;
        }

        let uv = Vec2::new(alpha, beta);

        let face = ray.get_face(outward_normal);

        let normal = match face {
            Face::Front => outward_normal,
            Face::Back => -outward_normal,
        };

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
