use crate::rendering::{Ray, HitRecord};
pub trait Intersect {
    fn intersect(r: Ray) -> HitRecord;
}