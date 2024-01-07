use vek::{Rgb, Vec2, Vec3};

#[derive(Debug, Clone, Default)]
pub struct Camera {
    pub position: Vec3<f32>,
    pub target: Vec3<f32>,
    pub up: Vec3<f32>,

    pub background_color: Rgb<f32>,
    pub vertical_fov: f32,
    pub defocus_angle: f32,
    pub focus_distance: f32,
}

pub struct Viewport {
    pub background_color: Rgb<f32>,
    pub origin: Vec3<f32>,
    pub upper_left_pixel_position: Vec3<f32>,

    pub horizontal_pixel_delta: Vec3<f32>,
    pub vertical_pixel_delta: Vec3<f32>,

    pub horizontal_defocus_disk: Vec3<f32>,
    pub vertical_defocus_disk: Vec3<f32>,
}

pub fn calculate_viewport(camera: Camera, image_size: Vec2<u32>) -> Viewport {
    let aspect_ratio = (image_size.x as f32) / (image_size.y as f32);

    let h = f32::tan(camera.vertical_fov / 2.);
    let height = 2.0 * h * camera.focus_distance;
    let width = height * aspect_ratio;

    let w = (camera.position - camera.target).normalized();
    let u = Vec3::cross(camera.up, w).normalized();
    let v = Vec3::cross(w, u);

    let horizontal = width * u;
    let vertical = -height * v;

    let horizontal_pixel_delta = horizontal / (image_size.x as f32);
    let vertical_pixel_delta = vertical / (image_size.y as f32);

    let upper_left_corner =
        camera.position - (camera.focus_distance * w) - horizontal / 2. - vertical / 2.;

    let upper_left_pixel_position =
        upper_left_corner + horizontal_pixel_delta / 2. + vertical_pixel_delta / 2.;

    let defocus_radius = camera.focus_distance * f32::tan(camera.defocus_angle / 2.);
    let horizontal_defocus_disk = u * defocus_radius;
    let vertical_defocus_disk = v * defocus_radius;

    Viewport {
        background_color: camera.background_color,
        origin: camera.position,
        upper_left_pixel_position,

        horizontal_pixel_delta,
        vertical_pixel_delta,

        horizontal_defocus_disk,
        vertical_defocus_disk,
    }
}
