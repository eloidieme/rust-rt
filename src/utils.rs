#![allow(dead_code)]

use std::f64::consts::PI;

use rand::Rng;

use crate::vec3::Vec3;

pub fn degrees_to_radians(angle: f64) -> f64 {
    angle * PI / 180.0
}

pub fn random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn random_offset_vector() -> Vec3<f64> {
    Vec3::new(random_float(0.0, 0.5), random_float(0.0, 0.5), 0.0)
}
