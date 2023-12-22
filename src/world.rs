use std::ops::Range;
use vek::geom::repr_simd::Ray;

use crate::data::{RayHit, Shape};

pub type World = Vec<Box<dyn Shape + Send + Sync>>;

impl Shape for World {
    fn hit(&self, ray: Ray<f32>, range: Range<f32>) -> Option<RayHit> {
        let mut closest_hit = None;
        let mut closest_distance = range.end;

        for hittable in self {
            if let Some(hit) = hittable.hit(ray, range.start..closest_distance) {
                closest_hit = Some(hit.clone());
                closest_distance = hit.distance;
            }
        }

        closest_hit
    }
}
