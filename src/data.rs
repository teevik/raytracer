use crate::{interval::Interval, materials::Material};
use vek::{Ray, Rgb, Vec3};

pub trait Raycastable {
    fn raycast(&self, ray: Ray<f32>, interval: Interval) -> Option<RayHit>;
}

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
    pub material: Material,
}

#[derive(Debug, Clone)]
pub struct ScatterResult {
    /// The new ray
    pub scattered: Ray<f32>,

    /// The color produced from scattering
    pub attenuation: Rgb<f32>,
}
