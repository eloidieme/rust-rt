#![allow(dead_code)]

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};
use std::boxed::Box;

pub struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        let hittables: Vec<Box<_>> = Vec::new();
        HittableList { hittables }
    }

    pub fn add<T: Hittable + 'static>(&mut self, hittable: T) {
        self.hittables.push(Box::new(hittable));
    }

    pub fn clear(&mut self) {
        self.hittables.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut current_interval: Interval = interval;
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
