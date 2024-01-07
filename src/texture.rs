use image::Rgb32FImage;
use noise::NoiseFn;
use std::fmt::Debug;
use std::sync::Arc;
use vek::{Rgb, Vec2, Vec3};

pub trait Noise: NoiseFn<f64, 3> + Debug + Sync + Send {}

impl<T: NoiseFn<f64, 3> + Debug + Sync + Send> Noise for T {}

#[derive(Debug, Clone)]
pub enum Texture {
    Solid {
        color: Rgb<f32>,
    },

    Checker {
        even: Rgb<f32>,
        odd: Rgb<f32>,
        inverse_scale: f32,
    },

    Noise {
        noise: Arc<dyn Noise>,
        scale: f32,
    },

    Image {
        image: Arc<Rgb32FImage>,
    },
}

impl Texture {
    pub fn solid(color: Rgb<f32>) -> Self {
        Self::Solid { color }
    }

    pub fn checker(even: Rgb<f32>, odd: Rgb<f32>, scale: f32) -> Self {
        Self::Checker {
            even,
            odd,
            inverse_scale: 1. / scale,
        }
    }

    pub fn noise(noise: Arc<dyn Noise>, scale: f32) -> Self {
        Self::Noise { noise, scale }
    }

    pub fn image(image: Arc<Rgb32FImage>) -> Self {
        Self::Image { image }
    }

    pub fn color_at(&self, uv: Vec2<f32>, point: Vec3<f32>) -> Rgb<f32> {
        match self {
            &Texture::Solid { color } => color,

            &Texture::Checker {
                inverse_scale,
                even,
                odd,
            } => {
                let point = (point * inverse_scale).floor().as_::<i32>();
                let is_even = point.sum() % 2 == 0;

                if is_even {
                    even
                } else {
                    odd
                }
            }

            Texture::Noise { noise, scale } => {
                let strength = noise.get((point * *scale).as_::<f64>().into_array()) as f32;
                let strength = strength + 1.;
                let strength = strength / 2.;

                strength * Rgb::one()
            }

            Texture::Image { image } => {
                let u = uv.x.clamp(0., 1.);
                let v = 1. - uv.y.clamp(0., 1.);

                let uv = Vec2::new(u, v);

                let image_size = Vec2::new(image.width(), image.height()).as_::<f32>();
                let pixel_position = (uv * image_size).as_::<u32>();

                let pixel = image.get_pixel(pixel_position.x, pixel_position.y);

                pixel.0.into()
            }
        }
    }
}
