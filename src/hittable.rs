use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Default)]
pub struct HitRecord {
    pub t: f64,
    pub normal: Vec3<f64>,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
