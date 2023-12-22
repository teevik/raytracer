use std::ops::Range;
use vek::Ray;

use crate::{data::RayHit, sphere::Sphere};

pub struct World(pub Vec<Sphere>);

impl World {
    pub fn raytrace(&self, ray: Ray<f32>, range: Range<f32>) -> Option<RayHit> {
        let mut closest_hit = None;
        let mut closest_distance = range.end;

        for sphere in &self.0 {
            if let Some(hit) = sphere.raytrace(ray, range.start..closest_distance) {
                closest_hit = Some(hit.clone());
                closest_distance = hit.distance;
            }
        }

        closest_hit
    }
}
