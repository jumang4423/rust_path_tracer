use super::super::vec3::vec3::Vec3;
use super::super::ray::ray::Ray;

fn color (mut ray: Ray) -> Vec3 {
    let unit_direction: Vec3 = ray.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) +  t * Vec3::new(0.5, 0.7, 1.0)
}

pub fn render(win_width: u64, win_height: u64) -> Vec<Vec3> {

    // pixel array of win_width * win_height
    let mut pixels: Vec<Vec3> = Vec::new();

    // init scene
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..=(win_height - 1)).rev() {
        for i in 0..win_width {
            let u = i as f32 / win_width as f32;
            let v = j as f32 / win_height as f32;
            let r = Ray::new(origin.clone(), lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v);
            let pixel_color = color(r);
            pixels.push(pixel_color);
        }
    }

    pixels
}