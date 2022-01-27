use super::super::ray::ray::Ray;
use super::super::vec3::vec3::Vec3;
use super::super::hitable::hitable::{Hitable, HitRecord};
use super::super::hitable_list::hitable_list::HitableList;
use super::super::camera::camera::Camera;

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

fn color(ray: &mut Ray, world: &HitableList) -> Vec3 {
    let mut rec: Box<HitRecord> = Box::new(HitRecord {
        t: 0.0,
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
    });
    if world.hit(ray.clone(), 0.001, f32::MAX, &mut rec) {
        let target: Vec3 = rec.clone().p + rec.clone().normal + random_in_unit_sphere();
        // throw new ray until it doesnt hit something
        return 0.5 * color(&mut Ray::new(rec.clone().p, target - rec.p), world);
    } else {
        let unit_direction: Vec3 = ray.direction().make_unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }

}

pub fn render(win_width: u64, win_height: u64, sampling: u64) -> Vec<Vec3> {
    // pixel array of win_width * win_height
    let mut pixels: Vec<Vec3> = Vec::new();

    // // init scene
    let camera: Camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    ) ;

    // hitables
    let hitable_list: Box<HitableList> = Box::new(HitableList::new(vec![
        Hitable::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Hitable::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
        // Hitable::new(Vec3::new(1.0, 0.0, -1.0), 0.5),
        // Hitable::new(Vec3::new(-1.0, 0.0, -1.0), 0.5),
    ]));

    for j in (0..=(win_height - 1)).rev() {
        for i in 0..win_width {
            let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..sampling {
                let u: f32 = (i as f32 + rand::random::<f32>()) / win_width as f32;
                let v: f32 = (j as f32 + rand::random::<f32>()) / win_height as f32;
                let ray: Ray = camera.get_ray(u, v);
                col += color(&mut ray.clone(), &hitable_list);
            }
            col /= sampling as f32;
            pixels.push(col);
        }
    }

    pixels
}
