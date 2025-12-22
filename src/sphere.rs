use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

/// A sphere object in the scene.
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    /// Creates a new Sphere with a center, radius, and material.
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, bounds: Interval) -> Option<HitRecord> {
        let oc: Vec3 = self.center - ray.origin;
        let a: f64 = ray.direction.dot(ray.direction);
        let h: f64 = oc.dot(ray.direction);
        let c: f64 = oc.dot(oc) - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd: f64 = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if !bounds.surrounds(root) {
            root = (h + sqrtd) / a;
            if !bounds.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;

        Some(HitRecord::new(p, normal, root, ray, self.material.clone()))
    }
}
