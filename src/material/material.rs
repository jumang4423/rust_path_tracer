use super::super::hitable::hitable::{HitRecord};
use super::super::ray::ray::Ray;
use super::super::vec3::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn scatter(&self, r_in: &mut Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(metal) => metal.scatter(r_in, rec, attenuation, scattered)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }

    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p.clone() + rec.clone().normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p.clone(), target - rec.p.clone());
        *attenuation = self.albedo.clone();

        true // always
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }

    fn scatter(
        &self,
        r_in: &mut Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(r_in.direction(), rec.normal.clone());
        *scattered = Ray::new(rec.p.clone(), reflected);
        *attenuation = self.albedo.clone();

        scattered.direction().dot(rec.clone().normal) > 0.0
    }
}

// -1 ~ 1
fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() >= 1.0 {
            break;
        }
    }
    p
}

fn reflect(mut v: Vec3, n: Vec3) -> Vec3 {
    v.clone() - 2.0 * v.dot(n.clone()) * n
}