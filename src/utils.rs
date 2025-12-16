#![allow(dead_code)]

use std::f64::consts::PI;

pub fn degrees_to_radians(angle: f64) -> f64 {
    angle * PI / 180.0
}
