
mod ppm_converter;
mod color;

use std::fs::File;

fn main() {

    // load json from settings.json
    let file = File::open("settings.json").expect("cannot find settings.json");
    let json: serde_json::Value = serde_json::from_reader(file)
    .expect("file should be proper JSON");

    // get settings
    let win_width: u32 = json["width"].as_u64().unwrap() as u32;
    let win_height: u32 = json["height"].as_u64().unwrap() as u32;
    let ppm_file_name: &str = json["ppm_file_name"].as_str().unwrap();

    // convert result to ppm
    let mut ppm_converter = ppm_converter::ppm_converter::PpmConverter::new(win_width, win_height, vec![]);
    match ppm_converter.export_ppm_as_file(ppm_file_name) {
        Ok(_) => println!("ppm generated"),
        Err(e) => println!("Error: {}", e)
    }
}
