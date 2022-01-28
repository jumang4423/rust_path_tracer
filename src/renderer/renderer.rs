use super::super::camera::camera::Camera;
use super::super::hitable::hitable::HitRecord;
use super::super::hitable_list::hitable_list::HitableList;
use super::super::material::material::{Lambertian, Material};
use super::super::ray::ray::Ray;
use super::super::vec3::vec3::Vec3;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq, Clone)]
struct ThreadData {
    thread_id: i32,
    pixels: Vec<Vec3>,
}

fn color(ray: &mut Ray, world: &HitableList, depth: i32) -> Vec3 {
    let mut rec: Box<HitRecord> = Box::new(HitRecord {
        t: 0.0,
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        material: Box::new(Material::Lambertian(Lambertian::new(Vec3::new(
            0.0, 0.0, 0.0,
        )))),
    });
    if world.hit(ray.clone(), 0.001, f32::MAX, &mut rec) {
        let mut scattered: Ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        // scatter from rec.material
        if depth < 32
            && rec
                .clone()
                .material
                .scatter(ray, &mut *rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&mut scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0); // return vec of record
        }
    } else {
        let unit_direction: Vec3 = ray.direction().make_unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

pub fn render(
    win_width: u64,
    win_height: u64,
    sampling: u64,
    multi_thread_num: i32,
    camera: Arc<Mutex<Camera>>,
    world: Arc<Mutex<HitableList>>,
) -> Vec<Vec3> {
    let (sender, receiver) = mpsc::channel();

    for t in 0..(multi_thread_num) {
        // multi-thread rendering
        let sender_clone = sender.clone();
        let camera_clone = camera.clone();
        let world_clone = world.clone();
        std::thread::spawn(move || {
            let mut pixels: Vec<Vec3> = Vec::new();

            for j in (t * (win_height as i32 / multi_thread_num))
                ..((t + 1) * (win_height as i32 / multi_thread_num))
            {
                for i in 0..(win_width) {
                    let mut color_v3: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..sampling {
                        let u: f32 = (i as f32 + rand::random::<f32>()) / win_width as f32;
                        let v: f32 = (j as f32 + rand::random::<f32>()) / win_height as f32;
                        let mut ray: Ray = camera_clone.lock().unwrap().get_ray(u, v);
                        color_v3 = color_v3 + color(&mut ray, &*world_clone.lock().unwrap(), 0);
                    }
                    color_v3 /= sampling as f32;
                    pixels.push(color_v3);
                }
            }

            let send_data: ThreadData = ThreadData {
                thread_id: t as i32,
                pixels,
            };

            println!("thread {} finished", t + 1);

            sender_clone.send(send_data).unwrap();
        });
    }

    let mut reveived_data: Vec<ThreadData> = Vec::new();
    for _ in 0..multi_thread_num {
        let recv_data: ThreadData = receiver.recv().unwrap();
        reveived_data.push(recv_data);
    }

    // sort reveived_data by thread_id
    println!("- merging data...");
    reveived_data.sort_by(|a, b| a.thread_id.cmp(&b.thread_id));

    let mut pixels: Vec<Vec3> = Vec::new();
    for t in 0..multi_thread_num {
        for pixel in reveived_data[t as usize].pixels.clone() {
            pixels.push(pixel);
        }
    }

    println!("{}", pixels.len());

    pixels
}
