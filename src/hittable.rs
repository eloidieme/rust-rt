use std::rc::Rc;

use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, normal: Vec3) {
        if (normal.dot(r.direction())).is_sign_positive() {
            self.front_face = false;
            self.normal = -normal;
        } else {
            self.normal = normal;
            self.front_face = true;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, bounds: Interval) -> Option<HitRecord>;
}
