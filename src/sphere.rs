use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{Vec3, dot},
};

#[derive(Debug)]
pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3<f64>, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc: Vec3<f64> = self.center - ray.origin();
        let a: f64 = dot(ray.direction(), ray.direction());
        let h: f64 = dot(oc, ray.direction());
        let c: f64 = dot(oc, oc) - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut rec = HitRecord::default();

        let sqrtd: f64 = discriminant.sqrt();

        let t1 = (h - sqrtd) / a;
        let t2 = (h + sqrtd) / a;

        if t1 >= tmin && t1 <= tmax {
            rec.t = t1;
        } else if t2 >= tmin && t2 <= tmax {
            rec.t = t2;
        } else {
            return None;
        }

        let normal: Vec3<f64> = ray.at(rec.t) - self.center;
        if (normal * ray.direction()).is_sign_positive() {
            rec.front_face = false;
            rec.normal = -normal / self.radius;
        } else {
            rec.normal = normal / self.radius;
            rec.front_face = true;
        }

        return Some(rec);
    }
}
