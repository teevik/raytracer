use crate::{bvh::Aabb, interval::Interval, materials::Material};
use vek::{Rgb, Vec2, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3<f32>,
    pub direction: Vec3<f32>,
}

impl Ray {
    pub fn new(origin: Vec3<f32>, direction: Vec3<f32>) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f32) -> Vec3<f32> {
        self.origin + (self.direction * t)
    }

    pub fn get_face(self, outward_normal: Vec3<f32>) -> Face {
        let direction = Vec3::dot(self.direction, outward_normal);

        if direction < 0. {
            Face::Front
        } else {
            Face::Back
        }
    }
}

pub trait Hittable {
    fn bounding_box(&self) -> Aabb;

    fn raycast(&self, ray: Ray, interval: Interval) -> Option<RayHit>;
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

    /// Texture coordinate
    pub uv: Vec2<f32>,

    /// The material of the hit shape
    pub material: Material,
}

#[derive(Debug, Clone)]
pub struct ScatterResult {
    /// The new ray
    pub scattered: Ray,

    /// The color produced from scattering
    pub attenuation: Rgb<f32>,
}
