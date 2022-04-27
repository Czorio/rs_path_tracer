use crate::math::Vec3;
use std::ops;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn get_point_at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
