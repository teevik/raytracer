use crate::camera::Camera;
use crate::sphere::Sphere;

pub mod scene_1;
pub mod scene_2;
pub mod scene_3;

pub struct Scene {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
}
