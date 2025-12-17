use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    utils::{random_unit_vector, reflect},
};

#[derive(Debug)]
pub struct ScatteredRay {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay>;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some(ScatteredRay {
            attenuation: self.albedo,
            scattered,
        })
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let reflected = reflect(r_in.direction(), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        Some(ScatteredRay {
            attenuation: self.albedo,
            scattered,
        })
    }
}
