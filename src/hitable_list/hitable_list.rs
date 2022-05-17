use super::super::hitable::hitable::{HitRecord, Hitable};
use super::super::material::material::{Lambertian, Material, Metal, Dielectric};
use super::super::ray::ray::Ray;
use super::super::vec3::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct HitableList {
    pub list: Vec<Hitable>,
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

pub fn hundred_spheres() -> HitableList {
    let mut world: HitableList = HitableList::new(vec![]);
    // put random 128 shere in the  hitable_list
    for _ in 0..128 {
        let rng_x: f32 = rand::random::<f32>() - 0.5;
        let rng_y: f32 = rand::random::<f32>() - 0.5;
        let rng_z: f32 = -rand::random::<f32>();
        let rng_radius: f32 = rand::random::<f32>() / 8.0;
        let rng_fuzz = rand::random::<f32>() / 2.0;
        let rng_material: Material = Material::Dielectric(Dielectric::new(rng_fuzz));
        world.list.push(Hitable::new(
            Vec3::new(rng_x, rng_y, rng_z),
            rng_radius,
            Box::new(rng_material.clone()),
        ));

        world.list.push(Hitable::new(
            Vec3::new(rng_x, rng_y, rng_z),
            -rng_radius + 0.1,
            Box::new(rng_material),
        ));
    }

    world
}

pub fn one_sphere_in_the_room() -> HitableList {
    HitableList::new(vec![
        Hitable::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.35,
            Box::new(Material::Metal(Metal::new(Vec3::new(
                1.0, 0.7, 0.7,
            ), 0.0))),
        ),
        Hitable::new(
            Vec3::new(-0.5, 0.0, -1.0),
            0.30,
            Box::new(Material::Metal(Metal::new(Vec3::new(
                0.7, 1.0, 0.7,
            ), 0.25))),
        ),
        Hitable::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.25,
            Box::new(Material::Metal(Metal::new(Vec3::new(
                0.7, 0.7, 1.0,
            ), 0.5))),
        ),
        Hitable::new(
            Vec3::new(0.5, 0.0, -1.0),
            0.20,
            Box::new(Material::Metal(Metal::new(Vec3::new(
                1.0, 0.7, 1.0,
            ), 0.75))),
        ),
        Hitable::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.15,
            Box::new(Material::Metal(Metal::new(Vec3::new(
                0.7, 1.0, 1.0,
            ), 1.0))),
        ),

        Hitable::new(
            Vec3::new(0.0, 0.0, -10.0),
            7.65,
            Box::new(Material::Metal(Metal::new(Vec3::new(
                1.0, 1.0, 0.0,
            ), 0.0))),
        ),
        Hitable::new(
            Vec3::new(10.0, 0.0, -1.0),
            8.75,
            Box::new(Material::Lambertian(Lambertian::new(Vec3::new(
                0.5, 1.0, 0.5,
            )))),
        ),
        Hitable::new(
            Vec3::new(-10.0, 0.0, -1.0),
            8.75,
            Box::new(Material::Metal(Metal::new(Vec3::new(
                0.5, 0.5, 0.5,
            ), 0.7))),
        ),
    ])
}

pub fn every_materials() -> HitableList {
    HitableList::new(vec![
        Hitable::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Material::Metal(Metal::new(Vec3::new(
                1.0, 0.7, 0.7,
            ), 0.0))),
        ),
        Hitable::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Material::Lambertian(Lambertian::new(Vec3::new(
                1.0, 0.7, 0.7,
            )))),
        ),
        Hitable::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Material::Dielectric(Dielectric::new(2.5))),
        ),
        Hitable::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Box::new(Material::Dielectric(Dielectric::new(2.5))),
        ),
        // under wall
        Hitable::new(
            Vec3::new(0.0, 100.5, -1.0),
            100.0,
            Box::new(Material::Lambertian(Lambertian::new(Vec3::new(
                0.8, 0.8, 0.0,
            )))),
        ),
    ])
}