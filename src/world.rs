use std::ops::Range;
use vek::geom::repr_simd::Ray;

use crate::{
    data::{RayHit, Shape},
    sphere::Sphere,
};

pub type World = Vec<Sphere>;

impl Shape for World {
    fn hit(&self, ray: Ray<f32>, range: Range<f32>) -> Option<RayHit> {
        let mut closest_hit = None;
        let mut closest_distance = range.end;

        for sphere in self {
            if let Some(hit) = sphere.hit(ray, range.start..closest_distance) {
                closest_hit = Some(hit.clone());
                closest_distance = hit.distance;
            }
        }

        closest_hit
    }
}
