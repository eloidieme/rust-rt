use std::fs;
use std::sync::Arc;

use crate::{
    geometry::{hittable_list::HittableList, sphere::Sphere},
    imaging::{
        camera::Camera,
        canvas::Canvas,
        material::{Dielectric, Lambertian, Material, Metal},
        renderer::Renderer,
    },
    scene::{Background, MaterialConfig, ObjectConfig, SceneConfig},
};

pub struct Engine {
    renderer: Renderer,
}

impl Engine {
    pub fn new(samples: u32, depth: u32) -> Self {
        Self {
            renderer: Renderer::new(samples, depth),
        }
    }

    pub fn load_scene_from_file(&self, path: &str) -> (HittableList, Camera, u32, u32, Background) {
        let yaml_data = fs::read_to_string(path).expect("Unable to read scene file");
        let config: SceneConfig = serde_yaml::from_str(&yaml_data).expect("Invalid scene YAML");

        let mut world = HittableList::default();

        for obj in config.objects {
            match obj {
                ObjectConfig::Sphere {
                    center,
                    radius,
                    material,
                } => {
                    let mat: Arc<dyn Material + Send + Sync> = match material {
                        MaterialConfig::Lambertian { albedo } => Arc::new(Lambertian::new(albedo)),
                        MaterialConfig::Metal { albedo, fuzz } => {
                            Arc::new(Metal::new(albedo, fuzz))
                        }
                        MaterialConfig::Dielectric { index } => Arc::new(Dielectric::new(index)),
                    };

                    world.add(Sphere::new(center, radius, mat));
                }
            }
        }

        let camera = Camera::builder()
            .aspect_ratio(config.aspect_ratio)
            .look_from(config.camera.look_from)
            .look_at(config.camera.look_at)
            .vup(config.camera.vup)
            .fov(config.camera.fov)
            .defocus_angle(config.camera.defocus_angle)
            .focus_dist(config.camera.focus_dist)
            .build();

        let height = (config.width as f64 / config.aspect_ratio) as u32;

        (world, camera, config.width, height, config.background)
    }

    pub fn render(
        &self,
        world: &HittableList,
        camera: &Camera,
        width: u32,
        height: u32,
        background: &Background,
    ) -> Canvas {
        let mut canvas = Canvas::new(width, width as f64 / height as f64);

        println!(
            "Rendering {}x{} image with {} samples...",
            width, height, self.renderer.samples_per_pixel
        );

        self.renderer.render(world, camera, &mut canvas, background);
        canvas
    }
}
