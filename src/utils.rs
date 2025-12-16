#![allow(dead_code)]

use std::f64::consts::PI;

use rand::Rng;

use crate::vec3::{Vec3, dot};

pub fn degrees_to_radians(angle: f64) -> f64 {
    angle * PI / 180.0
}

pub fn random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn random_offset_vector() -> Vec3<f64> {
    Vec3::new(random_float(-0.5, 0.5), random_float(-0.5, 0.5), 0.0)
}

pub fn random_unit_vector() -> Vec3<f64> {
    let mut result: Vec3<f64> = Vec3::new(
        random_float(-1.0, 1.0),
        random_float(-1.0, 1.0),
        random_float(-1.0, 1.0),
    );

    while result.length_squared() > 1.0 && result.length_squared() < 1e-160 {
        result = Vec3::new(
            random_float(-1.0, 1.0),
            random_float(-1.0, 1.0),
            random_float(-1.0, 1.0),
        );
    }

    result / result.length()
}

pub fn random_vector_on_hemisphere(normal: Vec3<f64>) -> Vec3<f64> {
    let on_unit_sphere = random_unit_vector();

    if dot(on_unit_sphere, normal) < 0.0 {
        -on_unit_sphere
    } else {
        on_unit_sphere
    }
}
