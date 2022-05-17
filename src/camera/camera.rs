use super::super::ray::ray::Ray;
use super::super::vec3::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,

}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, _focus_dist: f32) -> Camera {

        let mut init_camera: Camera = Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(0.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 0.0, 0.0),
            lens_radius: aperture / 2.0,
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0)
        };
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        init_camera.origin = Vec3::new(lookfrom.x(), lookfrom.y(), lookfrom.z());
        init_camera.w = (lookfrom - lookat).make_unit_vector();
        init_camera.u = vup.clone().cross(init_camera.w.clone()).make_unit_vector();
        init_camera.v = init_camera.w.clone().cross(init_camera.u.clone());
        init_camera.lower_left_corner = init_camera.origin.clone() - half_width * _focus_dist * init_camera.u.clone() - half_height * _focus_dist * init_camera.v.clone() - _focus_dist * init_camera.w.clone();
        init_camera.horizontal = 2.0 * half_width * init_camera.u.clone();
        init_camera.vertical = 2.0 * half_height * init_camera.v.clone();

        init_camera
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * random_in_unit_disk();
        let offset = self.u.clone() * rd.x() + self.v.clone() * rd.y();
        Ray::new(
            self.origin.clone() + offset.clone(),
            self.lower_left_corner.clone() + s * self.horizontal.clone() + t * self.vertical.clone() - self.origin.clone() - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    loop {
        p = Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(p.clone()) < 1.0 {
            break;
        }
    }
    p
}
