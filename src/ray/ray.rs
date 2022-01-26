use super::super::vec3::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn origin (&mut self) -> Vec3 {
        self.origin.clone()
    }

    pub fn direction (&mut self) -> Vec3 {
        self.direction.clone()
    }

    pub fn point_at_parameter(&mut self, t: f32) -> Vec3 {
        self.origin.clone() + self.direction.clone() * t
    }
}