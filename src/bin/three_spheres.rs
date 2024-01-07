use raytracer::camera::Camera;
use raytracer::materials::Material;
use raytracer::shapes::sphere::Sphere;
use raytracer::texture::Texture;
use raytracer::{render_image, Scene};
use vek::{Rgb, Vec3};

fn main() {
    let camera = Camera {
        position: Vec3::new(0., 0., 0.),
        target: Vec3::new(0., 0., -1.),
        up: Vec3::new(0., 1., 0.),

        background_color: Rgb::new(0.7, 0.8, 1.),
        vertical_fov: (90_f32).to_radians(),
        defocus_angle: (0_f32).to_radians(),
        focus_distance: 1.,
    };

    let spheres = vec![
        Sphere::new(
            Vec3::new(0., -100.5, -1.),
            100.,
            Material::Diffuse {
                albedo: Texture::solid(Rgb::new(0.8, 0.8, 0.)),
            },
        ),
        Sphere::new(
            Vec3::new(0., 0., -1.),
            0.5,
            Material::Diffuse {
                albedo: Texture::solid(Rgb::new(0.1, 0.2, 0.5)),
            },
        ),
        Sphere::new(
            Vec3::new(-1., 0., -1.),
            0.5,
            Material::Glass {
                refraction_index: 1.5,
            },
        ),
        Sphere::new(
            Vec3::new(-1., 0., -1.),
            -0.4,
            Material::Glass {
                refraction_index: 1.5,
            },
        ),
        Sphere::new(
            Vec3::new(1., 0., -1.),
            0.5,
            Material::Metal {
                albedo: Texture::solid(Rgb::new(0.8, 0.6, 0.2)),
                fuzz: 0.1,
            },
        ),
    ];

    let scene = Scene {
        camera,
        spheres,
        ..Default::default()
    };

    let image = render_image(scene);
    image.save("image.png").unwrap();
}
