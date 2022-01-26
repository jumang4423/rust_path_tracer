use super::super::ray::ray::Ray;
use super::super::vec3::vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f32, mut ray: Ray) -> Option<f32> {
    let mut oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction()).clone();
    let c = oc.dot(oc.clone()) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    // if sphere is behind the ray, no hit
    if discriminant < 0.0 {
        return None;
    }

    Some((-b - discriminant.sqrt()) / (a * 2.0))
}

fn color(mut ray: Ray) -> Vec3 {
    // if hit sphere, return the color of the sphere
    if let Some(t) = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray.clone()) {
        let n = (ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)).clone();
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }

    let unit_direction: Vec3 = ray.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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
            let r = Ray::new(
                origin.clone(),
                lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v,
            );
            let pixel_color = color(r);
            pixels.push(pixel_color);
        }
    }

    pixels
}
