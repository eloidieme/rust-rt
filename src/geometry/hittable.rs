use std::sync::Arc;

use crate::{
    imaging::material::Material,
    math::{interval::Interval, ray::Ray, vec3::Vec3},
};

/// Records details of a ray-object intersection.
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    /// Creates a new HitRecord, ensuring the normal points against the ray.
    pub fn new(p: Vec3, normal: Vec3, t: f64, ray: &Ray, material: Arc<dyn Material>) -> Self {
        let front_face = ray.direction.dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        HitRecord {
            t,
            p,
            normal,
            material,
            front_face,
        }
    }
}

/// Trait for objects that can be hit by a ray.
pub trait Hittable {
    /// Determines if a ray hits the object within the given interval.
    fn hit(&self, ray: &Ray, bounds: Interval) -> Option<HitRecord>;
}
