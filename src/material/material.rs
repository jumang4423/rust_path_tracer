use super::super::hitable::hitable::{HitRecord};
use super::super::ray::ray::Ray;
use super::super::vec3::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(&self, r_in: &mut Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(metal) => metal.scatter(r_in, rec, attenuation, scattered),
            Material::Dielectric(dielectric) => dielectric.scatter(r_in, rec, attenuation, scattered),
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
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Metal {
        Metal { albedo, fuzz: f.min(1.0).max(0.0) }
    }

    fn scatter(
        &self,
        r_in: &mut Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(r_in.direction(), rec.normal.clone());
        *scattered = Ray::new(rec.p.clone(), reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo.clone();

        scattered.direction().dot(rec.clone().normal) > 0.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ri: f32) -> Dielectric {
        Dielectric { ref_idx: ri }
    }

    pub fn scatter( 
        &self,
        r_in: &mut Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal: Vec3;
        let reflected: Vec3 = reflect(r_in.direction(), rec.normal.clone());
        let ni_over_nt: f32;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut reflect_prob: f32 = 0.0;
        let cosine: f32;
        if r_in.direction().dot(rec.normal.clone()) > 0.0 {
            outward_normal = rec.normal.clone() * -1.0;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction().dot(rec.normal.clone()) / r_in.direction().length();
        } else {
            outward_normal = rec.normal.clone();
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction().dot(rec.normal.clone()) / r_in.direction().length();
        }
        
        if refract( r_in.direction(), outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        let rnd = rand::random::<f32>();
        if rnd < reflect_prob {
            *scattered = Ray::new(rec.p.clone(), reflected.clone());
        } else {
            *scattered = Ray::new(rec.p.clone(), refracted);
        }
        
        true
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

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let mut uv = v.clone().make_unit_vector();
    let dt = uv.dot(n.clone());
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - n.clone() * dt) - n * discriminant.sqrt();
        true
    } else {
        false
    }
}

fn schlick(cos: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}