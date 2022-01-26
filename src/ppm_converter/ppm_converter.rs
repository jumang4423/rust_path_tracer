use super::super::vec3;
use std::io::prelude::*;

// PpmConverter is a struct that contains the data of a ppm file.
pub struct PpmConverter {
    width: u32,
    height: u32,
    points: Vec<Vec<vec3::vec3::Vec3>>
}

impl PpmConverter {
    // constructor
    pub fn new(width: u32, height: u32, points: Vec<Vec<vec3::vec3::Vec3>>) -> PpmConverter {
        PpmConverter {
            width,
            height,
            points
        }
    }

    pub fn ppm_draw_as_string(&mut self) -> String {
        let mut ppm_string = String::new();
        ppm_string.push_str(self.ppm_header().as_str());
        ppm_string.push_str(self.ppm_body().as_str());
        ppm_string
    }

    pub fn export_ppm_as_file(&mut self, file_name: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(file_name).unwrap();
        file.write_all(self.ppm_draw_as_string().as_bytes())
    }
    // privates
    // header info str
    fn ppm_header (&mut self) -> String {
        format!("P3\n{} {}\n255\n# ppm\n", self.width, self.height)
    }

    // body info str
    fn ppm_body (&mut self) -> String {
        let mut ppm_body = String::new();

        let att_points: Vec<Vec<vec3::vec3::Vec3>>;

        if self.points.len() == 0 {
            att_points = self.test_rainbow_ppm();
        } else {
            att_points = self.points.clone();
        }

        for row in att_points {
            for point in row.iter() {
                ppm_body.push_str(format!("{} {} {} ", point.r() as u8, point.g() as u8, point.b() as u8).as_str());
            }
            ppm_body.push_str("\n");
        }
        ppm_body
    }

    // draw rainbow
    fn test_rainbow_ppm(&mut self) -> Vec<Vec<vec3::vec3::Vec3>> {
        let mut rainbow_points = vec![];
        for i in 0..self.height {
            let mut row = vec![];
            for j in 0..self.width {
                let red = 255.0 / self.height as f32 * i as f32;
                let green = 255.0 / self.width as f32 * j as f32;
                let blue = 255.0 / self.height as f32 * i as f32;
                row.push(vec3::vec3::Vec3::new(red, green, blue));
            }
            rainbow_points.push(row);
        }
        rainbow_points
    }
}

