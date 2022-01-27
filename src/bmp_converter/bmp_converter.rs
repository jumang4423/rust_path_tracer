use super::super::vec3;
use bmp::{Image, Pixel};

pub struct BmpConverter {
    width: u64,
    height: u64,
    points: Vec<vec3::vec3::Vec3>,
}

impl BmpConverter {
    // constructor
    pub fn new(width: u64, height: u64, points: Vec<vec3::vec3::Vec3>) -> BmpConverter {
        BmpConverter {
            width,
            height,
            points,
        }
    }

    // convert to bmp

    pub fn export_bmp_as_file(&mut self, file_name: &str) -> std::io::Result<()> {
        let mut image = Image::new(self.width as u32, self.height as u32);
        for i in 0..self.points.len() {
            let point = &mut self.points[i];
            let pixel = Pixel::new(point.r() as u8, point.g() as u8, point.b() as u8);
            image.set_pixel(
                i as u32 % self.width as u32,
                i as u32 / self.width as u32,
                pixel,
            );
        }

        image.save(file_name.to_string())
    }
}
