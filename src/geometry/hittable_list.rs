use crate::{
    geometry::hittable::{HitRecord, Hittable},
    math::{interval::Interval, ray::Ray},
};
use std::boxed::Box;

#[derive(Default)]
pub struct HittableList {
    hittables: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn add<T: Hittable + 'static + Send + Sync>(&mut self, hittable: T) {
        self.hittables.push(Box::new(hittable));
    }

    pub fn _clear(&mut self) {
        self.hittables.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, bounds: Interval) -> Option<HitRecord> {
        let mut current_interval: Interval = bounds;
        let mut closest_hit: Option<HitRecord> = None;

        for hittable in &self.hittables {
            if let Some(rec) = hittable.hit(ray, current_interval) {
                current_interval.max = rec.t;
                closest_hit = Some(rec);
            }
        }

        closest_hit
    }
}
