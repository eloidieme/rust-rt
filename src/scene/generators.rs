use crate::{
    geometry::{hittable_list::HittableList, sphere::Sphere},
    imaging::{
        camera::Camera,
        material::{Dielectric, Lambertian, MaterialKind, Metal},
    },
    math::{
        utils,
        vec3::{Color, Vec3},
    },
};

/// Generates a random scene similar to the cover of "Ray Tracing in One Weekend".
pub fn random_book_scene(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let ground_material = MaterialKind::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random();
            let center = Vec3::new(
                a as f64 + 0.9 * utils::random(),
                0.2,
                b as f64 + 0.9 * utils::random(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: MaterialKind;

                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = MaterialKind::Lambertian(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = utils::random_range(0.0, 0.5);
                    sphere_material = MaterialKind::Metal(Metal::new(albedo, fuzz));
                } else {
                    // Glass
                    sphere_material = MaterialKind::Dielectric(Dielectric::new(1.5));
                }

                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material_1 = MaterialKind::Dielectric(Dielectric::new(1.5));
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material_1));

    let material_2 = MaterialKind::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material_2));

    let material_3 = MaterialKind::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material_3));

    let camera = Camera::builder()
        .aspect_ratio(aspect_ratio)
        .look_from(Vec3::new(13.0, 2.0, 3.0))
        .look_at(Vec3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .fov(20.0)
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build();

    (world, camera)
}
