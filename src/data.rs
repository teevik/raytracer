use std::{fmt::Debug, ops::Range, sync::Arc};
use vek::{geom::repr_simd::Ray, vec::repr_simd::Vec3};

#[derive(Debug, Clone, Copy)]
pub enum Face {
    Front,
    Back,
}

#[derive(Debug, Clone)]
pub struct RayHit {
    /// Distance to hit
    pub distance: f32,

    /// The point where the ray hit
    pub point: Vec3<f32>,

    /// Which face
    pub face: Face,

    /// Normal, unit length
    pub normal: Vec3<f32>,

    /// The material of the hit shape
    pub material: Arc<dyn Material>,
}

#[derive(Debug, Clone)]
pub struct ScatterResult {
    /// The new ray
    pub scattered: Ray<f32>,

    /// The color produced from scattering
    pub attenuation: Vec3<f32>,
}

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, ray: Ray<f32>, ray_hit: RayHit) -> Option<ScatterResult>;
}

pub trait Shape: Debug + Send + Sync {
    fn hit(&self, ray: Ray<f32>, range: Range<f32>) -> Option<RayHit>;
}
