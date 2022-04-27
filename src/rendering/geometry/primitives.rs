use crate::math::Vec3;
use crate::rendering::geometry::Intersect;
use crate::rendering::{HitRecord, Ray};

struct Sphere {
    center: Vec3,
    radius: f64
}

impl Intersect for Sphere {
    fn intersect(r: Ray) -> HitRecord {
        todo!()
    }
}



struct Triangle {
    v1: Vec3,
    v2: Vec3,
    v3: Vec3
}

impl Intersect for Triangle {
    fn intersect(r: Ray) -> HitRecord {
        todo!()
    }
}