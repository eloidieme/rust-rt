#![allow(dead_code)]

use crate::{
    hittable::{HitRecord, Hittable},
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
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far: f64 = tmax;
        let mut closest_hit: Option<HitRecord> = None;

        for hittable in &self.hittables {
            if let Some(rec) = hittable.hit(ray, tmin, closest_so_far) {
                closest_so_far = rec.t;
                closest_hit = Some(rec);
            }
        }

        closest_hit
    }
}
