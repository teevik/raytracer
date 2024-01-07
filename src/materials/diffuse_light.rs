use crate::texture::Texture;
use vek::{Rgb, Vec2, Vec3};

pub fn emit(strength: &Texture, uv: Vec2<f32>, point: Vec3<f32>) -> Rgb<f32> {
    strength.color_at(uv, point)
}
