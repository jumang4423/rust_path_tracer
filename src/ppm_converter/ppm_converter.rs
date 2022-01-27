use super::super::vec3;
use std::io::prelude::*;

// PpmConverter is a struct that contains the data of a ppm file.
pub struct PpmConverter {
    width: u64,
    height: u64,
    points: Vec<vec3::vec3::Vec3>
}

impl PpmConverter {
    // constructor
    pub fn new(width: u64, height: u64, points: Vec<vec3::vec3::Vec3>) -> PpmConverter {
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
        let mut file = std::fs::File::create(self.bmp_to_ppm_as_string(file_name.to_string())).unwrap();
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
        let att_points: Vec<vec3::vec3::Vec3>;

        if self.points.is_empty() {
            att_points = self.test_rainbow_ppm();
        } else {
            att_points = self.points.clone();
        }

        for point in att_points {
            ppm_body.push_str(format!("{} {} {}\n", point.r(), point.g(), point.b()).as_str());
        }

        ppm_body
    }

    // draw rainbow
    fn test_rainbow_ppm(&mut self) -> Vec<vec3::vec3::Vec3> {
        let mut rainbow_points = vec![];
        for i in 0..self.width {
            for j in 0..self.height {
                let point = vec3::vec3::Vec3::new(
                    (i as f32 / self.width as f32) * 255.0,
                    (j as f32 / self.height as f32) * 255.0,
                    0.0
                );
                rainbow_points.push(point);
            }
        }
        rainbow_points
    }


    fn bmp_to_ppm_as_string(&mut self, file_name: String) -> String {
        file_name.replace("bmp", "ppm")
    }
}

