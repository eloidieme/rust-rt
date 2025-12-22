use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};
use std::boxed::Box;

#[derive(Default)]
/// A list of hittable objects.
pub struct HittableList {
    hittables: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    /// Adds a hittable object to the list.
    pub fn add<T: Hittable + 'static + Send + Sync>(&mut self, hittable: T) {
        self.hittables.push(Box::new(hittable));
    }

    /// Clears the list of hittable objects.
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
