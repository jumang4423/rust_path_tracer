use super::super::ray::ray::Ray;
use super::super::vec3::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin.clone(),
            self.lower_left_corner.clone()
                + self.horizontal.clone() * u
                + self.vertical.clone() * v - self.origin.clone(),
        )
    }
}
