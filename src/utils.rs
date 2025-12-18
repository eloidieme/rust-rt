#![allow(dead_code)]

use std::f64::consts::PI;

use rand::Rng;

use crate::vec3::Vec3;

pub fn degrees_to_radians(angle: f64) -> f64 {
    angle * PI / 180.0
}

pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        return linear.sqrt();
    }

    return 0.0;
}

pub fn random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn random_offset_vector() -> Vec3 {
    Vec3::new(random_float(-0.5, 0.5), random_float(-0.5, 0.5), 0.0)
}

pub fn random_unit_vector() -> Vec3 {
    let mut result: Vec3 = Vec3::new(
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

    result.unit_vector()
}

pub fn random_vector_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();

    if Vec3::dot(on_unit_sphere, normal) < 0.0 {
        -on_unit_sphere
    } else {
        on_unit_sphere
    }
}

pub fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - normal * 2.0 * Vec3::dot(incident, normal)
}

pub fn refract(incident: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(Vec3::dot(-incident, normal), 1.0);
    let r_out_perp = (incident + normal * cos_theta) * etai_over_etat;
    let r_out_parallel = normal * -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared()));

    r_out_perp + r_out_parallel
}
