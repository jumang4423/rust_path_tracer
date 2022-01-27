use super::super::hitable::hitable::{HitRecord, Hitable};
use super::super::material::material::{Lambertian, Material};
use super::super::ray::ray::Ray;
use super::super::vec3::vec3::Vec3;

pub struct HitableList {
    list: Vec<Hitable>,
}

impl HitableList {
    pub fn new(list: Vec<Hitable>) -> HitableList {
        HitableList { list }
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: Box<HitRecord> = Box::new(HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Box::new(Material::Lambertian(Lambertian::new(Vec3::new(
                0.0, 0.0, 0.0,
            )))),
        });
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hitable in &self.list {
            if hitable.hit(ray.clone(), t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = *temp_rec.clone();
            }
        }
        hit_anything
    }
}
