use crate::{
    imaging::material::MaterialKind,
    math::{interval::Interval, ray::Ray, vec3::Vec3},
};

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: MaterialKind,
    pub front_face: bool,
}

impl HitRecord {
    /// Creates a new HitRecord, ensuring the normal points against the ray.
    pub fn new(p: Vec3, normal: Vec3, t: f64, ray: &Ray, material: MaterialKind) -> Self {
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, bounds: Interval) -> Option<HitRecord>;
}
