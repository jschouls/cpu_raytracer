use super::light::Light;
use super::material::{Dielectric, Lambertian, Material, Metal};
use super::shape::{Plane, Shape, Sphere};
use super::Camera;
use super::Vec3;
use crate::math::vector::Vector;

use std::sync::Arc;

pub struct Scene {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

extern crate rand;
use rand::Rng;

// Create the scene.
pub fn create_scene() -> Scene {
    // Materials

    // Reference counter because this can be shared with others and rays.
    let floor_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Vec3(0.5, 0.5, 0.5),
    });

    let sphere_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Vec3(0.1, 0.2, 0.5),
    });

    let metal_material: Arc<dyn Material> = Arc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.3));

    let dielectric_mat: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));

    let from = Vec3(-2.0, 2.0, 1.0);
    let look_at = Vec3(0.0, 0.0, -2.0);
    let look_dist = (from - look_at).length();
    Scene {
        objects: vec![
            Box::new(Plane::new(Vec3::up(), 0.5, &floor_material)),
            // Box::new(Plane::new(Vec3(0.0, 0.0, 1.0), 5.0, &back_material)),
            // Box::new(Plane::new(Vec3(1.0, 0.0, 0.0), 5.0, &back_material)),
            // Box::new(Plane::new(Vec3(-1.0, 0.0, 0.0), 5.0, &back_material)),
            Box::new(Sphere::new(Vec3(0.0, 0.0, -2.0), 0.5, &sphere_material)),
            Box::new(Sphere::new(Vec3(1.0, 0.0, -2.0), 0.5, &metal_material)),
            Box::new(Sphere::new(Vec3(-1.0, 0.0, -2.0), 0.5, &dielectric_mat)),
            //Box::new(Sphere::new(Vec3(-1.0, 0.0, -2.0), -0.45, &dielectric_mat)),
        ],
        lights: vec![],
        camera: Camera::set(
            from,
            look_at,
            Vec3::up(),
            20.0,
            800.0 / 600.0,
            2.0,
            look_dist,
        ),
    }
}

/*pub fn create_scene() -> Scene {
    let from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let look_dist = (from - look_at).length();
    let ratio = 800.0 / 600.0;

    // Materials
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Vec3(0.5, 0.5, 0.5),
    });

    let material1: Arc<dyn Material> = Arc::new(Dielectric { refract: 1.5 });
    let material2: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Vec3(0.4, 0.2, 0.1),
    });
    let material3: Arc<dyn Material> = Arc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0));

    let mut scene = Scene {
        objects: vec![
            Box::new(Plane::new(Vec3::up(), 0.0, &ground_material)),
            Box::new(Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, &material1)),
            Box::new(Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0, &material2)),
            Box::new(Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, &material3)),
        ],
        lights: vec![],
        camera: Camera::set(from, look_at, Vec3::up(), 20.0, ratio, 0.1, look_dist),
    };

    let mut rngs = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let rand_mat: f64 = rngs.gen();
            let rand_center: (f64, f64) = rngs.gen();
            let center = Vec3(
                a as f64 + 0.9 * rand_center.0,
                0.2,
                b as f64 + 0.9 * rand_center.1,
            );

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if rand_mat < 0.8 {
                    // Lambertian
                    let color1: (f64, f64, f64) = rngs.gen();
                    let color2: (f64, f64, f64) = rngs.gen();
                    let albedo = Vec3(
                        color1.0 * color2.0,
                        color1.1 * color2.1,
                        color1.2 * color2.2,
                    );
                    let mat: Arc<dyn Material> = Arc::new(Lambertian { albedo });
                    scene.objects.push(Box::new(Sphere::new(center, 0.2, &mat)));
                } else if rand_mat < 0.95 {
                    // metal
                    let albedo: Vec3 = Vec3(
                        rngs.gen_range(0.5, 1.0),
                        rngs.gen_range(0.5, 1.0),
                        rngs.gen_range(0.5, 1.0),
                    );
                    let fuzz = rngs.gen_range(0.0, 0.5);
                    let mat2: Arc<dyn Material> = Arc::new(Metal::new(albedo, fuzz));
                    scene
                        .objects
                        .push(Box::new(Sphere::new(center, 0.2, &mat2)));
                } else {
                    let mat3: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
                    scene
                        .objects
                        .push(Box::new(Sphere::new(center, 0.2, &mat3)));
                }
            }
        }
    }

    scene
}*/
