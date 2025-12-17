use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new<M: Material + 'static>(center: Vec3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material: Rc::new(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, bounds: Interval) -> Option<HitRecord> {
        let oc: Vec3 = self.center - ray.origin();
        let a: f64 = Vec3::dot(ray.direction(), ray.direction());
        let h: f64 = Vec3::dot(oc, ray.direction());
        let c: f64 = Vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t;
        let p;

        let sqrtd: f64 = discriminant.sqrt();

        let t1 = (h - sqrtd) / a;
        let t2 = (h + sqrtd) / a;

        if bounds.surrounds(t1) {
            t = t1;
            p = ray.at(t1);
        } else if bounds.surrounds(t2) {
            t = t2;
            p = ray.at(t2);
        } else {
            return None;
        }

        let mut rec = HitRecord {
            t,
            p,
            normal: Vec3::default(),
            material: self.material.clone(),
            front_face: false,
        };

        let normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, normal);

        return Some(rec);
    }
}
