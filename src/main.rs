mod bmp_converter;
mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ppm_converter;
mod ray;
mod renderer;
mod vec3;

// crates
use std::fs::File;
use std::sync::{Arc, Mutex};

fn main() {
    // load settings from settings.json
    let settings: Settings = get_settings_from_json();
    println!("- loaded settings:\n{:?}", settings);

    // render scene
    println!("- rendering scene...");

    let camera: Arc<Mutex<camera::camera::Camera>> =
        Arc::new(Mutex::new(camera::camera::Camera::new(
            vec3::vec3::Vec3::new(0.0, 0.0, 0.0),
            vec3::vec3::Vec3::new(-2.0, -1.0, -1.0),
            vec3::vec3::Vec3::new(4.0, 0.0, 0.0),
            vec3::vec3::Vec3::new(0.0, 2.0, 0.0),
        )));

    let world: Arc<Mutex<hitable_list::hitable_list::HitableList>> =
        Arc::new(Mutex::new(hitable_list::hitable_list::every_materials()));

    let pixels = renderer::renderer::render(
        settings.win_width,
        settings.win_height,
        settings.sampling,
        settings.multi_thread_num,
        camera,
        world,
    );
    println!("- rendered scene");

    // save bmp file
    let mut bmp_converter = bmp_converter::bmp_converter::BmpConverter::new(
        settings.win_width,
        settings.win_height,
        pixels.clone(),
    );
    match bmp_converter.export_bmp_as_file(settings.bmp_file_name.as_str()) {
        Ok(_) => println!("- bmp generated"),
        Err(e) => println!("-! Error: {}", e),
    }

    // convert result to ppm if needed
    if settings.is_gen_ppm {
        let mut ppm_converter = ppm_converter::ppm_converter::PpmConverter::new(
            settings.win_width,
            settings.win_height,
            pixels,
        );
        match ppm_converter.export_ppm_as_file(settings.bmp_file_name.as_str()) {
            Ok(_) => println!("- ppm generated"),
            Err(e) => println!("-! Error: {}", e),
        }
    } else {
        println!("-o Warning: ppm generation is disabled");
    }
}

// load json then return settings
#[derive(Debug)]
struct Settings {
    win_width: u64,
    win_height: u64,
    sampling: u64,
    multi_thread_num: i32,
    bmp_file_name: String,
    is_gen_ppm: bool,
}

fn get_settings_from_json() -> Settings {
    // load json from settings.json
    let file = File::open("settings.json").expect("cannot find settings.json");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    // get settings
    let win_width: u64 = json["width"].as_u64().unwrap() as u64;
    let win_height: u64 = json["height"].as_u64().unwrap() as u64;
    let sampling: u64 = json["sampling"].as_u64().unwrap() as u64;
    let multi_thread_num: i32 = json["multi_thread_num"].as_u64().unwrap() as i32;
    let bmp_file_name: &str = json["bmp_file_name"].as_str().unwrap();
    let is_gen_ppm: bool = json["is_gen_ppm"].as_bool().unwrap();

    // return settings struct
    Settings {
        win_width,
        win_height,
        sampling,
        multi_thread_num,
        is_gen_ppm,
        bmp_file_name: bmp_file_name.to_string(),
    }
}
