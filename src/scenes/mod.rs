use crate::camera::Camera;
use crate::sphere::Sphere;

pub mod earth;
pub mod scene_1;
pub mod scene_2;
pub mod scene_3;
pub mod scene_4;
pub mod two_perlin_spheres;

pub struct Scene {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
}
