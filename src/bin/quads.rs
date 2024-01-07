use raytracer::camera::Camera;
use raytracer::materials::Material;
use raytracer::shapes::quad::Quad;
use raytracer::texture::Texture;
use raytracer::{render_image, Scene};
use vek::{Rgb, Vec3};

fn main() {
    let camera = Camera {
        position: Vec3::new(0., 0., 9.),
        target: Vec3::new(0., 0., 0.),
        up: Vec3::new(0., 1., 0.),

        background_color: Rgb::new(0.7, 0.8, 1.),
        vertical_fov: (80_f32).to_radians(),
        defocus_angle: (0_f32).to_radians(),
        focus_distance: 1.,
    };

    let left_red = Material::Diffuse {
        albedo: Texture::solid(Rgb::new(1.0, 0.2, 0.2)),
    };
    let back_green = Material::Diffuse {
        albedo: Texture::solid(Rgb::new(0.2, 1.0, 0.2)),
    };
    let right_blue = Material::Diffuse {
        albedo: Texture::solid(Rgb::new(0.2, 0.2, 1.0)),
    };
    let upper_orange = Material::Diffuse {
        albedo: Texture::solid(Rgb::new(1.0, 0.5, 0.0)),
    };
    let lower_teal = Material::Diffuse {
        albedo: Texture::solid(Rgb::new(0.2, 0.8, 0.8)),
    };

    let quads = vec![
        Quad::new(
            Vec3::new(-3., -2., 5.),
            Vec3::new(0., 0., -4.),
            Vec3::new(0., 4., 0.),
            left_red,
        ),
        Quad::new(
            Vec3::new(-2., -2., 0.),
            Vec3::new(4., 0., 0.),
            Vec3::new(0., 4., 0.),
            back_green,
        ),
        Quad::new(
            Vec3::new(3., -2., 1.),
            Vec3::new(0., 0., 4.),
            Vec3::new(0., 4., 0.),
            right_blue,
        ),
        Quad::new(
            Vec3::new(-2., 3., 1.),
            Vec3::new(4., 0., 0.),
            Vec3::new(0., 0., 4.),
            upper_orange,
        ),
        Quad::new(
            Vec3::new(-2., -3., 5.),
            Vec3::new(4., 0., 0.),
            Vec3::new(0., 0., -4.),
            lower_teal,
        ),
    ];

    let scene = Scene {
        camera,
        quads,
        ..Default::default()
    };
    let image = render_image(scene);
    image.save("image.png").unwrap();
}
