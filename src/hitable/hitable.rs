use super::super::ray::ray::Ray;
use super::super::vec3::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}



pub struct Hitable {
    center: Vec3,
    radius: f32,
}

impl Hitable {
    pub fn new(center: Vec3, radius: f32) -> Hitable {
        Hitable { center, radius }
    }

    pub fn hit(&self, mut ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut oc = ray.origin() - self.center.clone();
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc.clone()) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        // if sphere is behind the ray, no hit
        if discriminant > 0.0 {
            let temp: f32 = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                return true;
            }
        }
        false
    }
}
