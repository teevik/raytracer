use noise::{Perlin, Turbulence};
use raytracer::camera::Camera;
use raytracer::materials::Material;
use raytracer::shapes::quad::Quad;
use raytracer::shapes::sphere::Sphere;
use raytracer::texture::Texture;
use raytracer::{render_image, Scene};
use std::sync::Arc;
use vek::{Rgb, Vec3};

fn main() {
    let camera = Camera {
        position: Vec3::new(26., 3., 6.),
        target: Vec3::new(0., 2., 0.),
        up: Vec3::new(0., 1., 0.),

        background_color: Rgb::zero(),
        vertical_fov: (20_f32).to_radians(),
        defocus_angle: (0_f32).to_radians(),
        focus_distance: 1.,
    };

    // let perlin = Perlin::new(0);
    let perlin = Turbulence::<_, Perlin>::new(Perlin::new(0));

    let perlin_material = Material::Diffuse {
        albedo: Texture::noise(Arc::new(perlin), 5.),
    };

    let light_material = Material::DiffuseLight {
        strength: Texture::solid(Rgb::new(4., 4., 4.)),
    };

    let spheres = vec![
        Sphere::new(Vec3::new(0., -1000., 0.), 1000., perlin_material.clone()),
        Sphere::new(Vec3::new(0., 7., 0.), 2., light_material.clone()),
        Sphere::new(Vec3::new(0., 2., 0.), 2., perlin_material),
    ];

    let quads = vec![Quad::new(
        Vec3::new(3., 1., -2.),
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 2., 0.),
        light_material,
    )];

    let scene = Scene {
        camera,
        spheres,
        quads,
    };

    let image = render_image(scene);
    image.save("image.png").unwrap();
}
