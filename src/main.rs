mod ppm_converter;
mod ray;
mod renderer;
mod vec3;
mod hitable;
mod hitable_list;

// crates
use std::fs::File;

fn main() {
    // load settings from settings.json
    let settings: Settings = get_settings_from_json();
    println!("- loaded settings:\n{:?}", settings);

    // convert result to ppm
    let mut ppm_converter = ppm_converter::ppm_converter::PpmConverter::new(
        settings.win_width,
        settings.win_height,
        renderer::renderer::render(settings.win_width, settings.win_height),
    );
    match ppm_converter.export_ppm_as_file(settings.ppm_file_name.as_str()) {
        Ok(_) => println!("- ppm generated"),
        Err(e) => println!("-! Error: {}", e),
    }
}

// load json then return settings
#[derive(Debug)]
struct Settings {
    win_width: u64,
    win_height: u64,
    ppm_file_name: String,
}

fn get_settings_from_json() -> Settings {
    // load json from settings.json
    let file = File::open("settings.json").expect("cannot find settings.json");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    // get settings
    let win_width: u64 = json["width"].as_u64().unwrap() as u64;
    let win_height: u64 = json["height"].as_u64().unwrap() as u64;
    let ppm_file_name: &str = json["ppm_file_name"].as_str().unwrap();

    // return settings struct
    Settings {
        win_width,
        win_height,
        ppm_file_name: ppm_file_name.to_string(),
    }
}
