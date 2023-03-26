use super::light::Light;
use super::material::*;
use super::shape;
use super::shape::{Object, ObjectType};
use super::Camera;
use super::Vec3;
use crate::material;
use crate::math::vector::Vector;

#[derive(Clone)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

/*pub fn create_scene() -> Scene {
    let ground_material = material::new(Vec3(0.5, 0.5, 0.5), MaterialType::Lambertian);
    let sphere_material = material::new(Vec3(0.1, 0.2, 0.5), MaterialType::Lambertian);

    let metal_material = material::new(Vec3(0.8, 0.6, 0.2), MaterialType::Metal { fuzz: 0.3 });
    let dielectric_mat = material::new(Vec3::zero(), MaterialType::Dielectric { refract: 1.5 });

    let from = Vec3(-6.0, 1.5, 0.70);
    let look_at = Vec3(0.0, 0.0, -2.0);
    let look_dist = (from - look_at).length();
    Scene {
        objects: vec![
            shape::new(
                Vec3(0.0, 0.0, 0.0),
                ObjectType::Plane {
                    distance: (0.5),
                    normal: (Vec3::up()),
                },
                &ground_material,
            ),
            shape::new(
                Vec3(0.0, 0.0, -2.0),
                ObjectType::Sphere { radius: (0.5) },
                &sphere_material,
            ),
            shape::new(
                Vec3(1.0, 0.0, -2.0),
                ObjectType::Sphere { radius: (0.5) },
                &metal_material,
            ),
            shape::new(
                Vec3(-1.0, 0.0, -2.0),
                ObjectType::Sphere { radius: (0.5) },
                &dielectric_mat,
            ),
        ],
        lights: vec![],
        camera: Camera::set(
            from,
            look_at,
            Vec3::up(),
            20.0,
            super::SCREEN_WIDTH as f64 / super::SCREEN_HEIGHT as f64,
            0.7,
            look_dist,
        ),
    }
}*/

extern crate rand;
use rand::Rng;

pub fn create_scene() -> Scene {
    let from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let look_dist = (from - look_at).length();
    let ratio = super::SCREEN_WIDTH as f64 / super::SCREEN_HEIGHT as f64;

    // Materials

    let ground_material = material::new(Vec3(0.5, 0.5, 0.5), MaterialType::Lambertian);
    let material1 = material::new(Vec3::zero(), MaterialType::Dielectric { refract: 1.5 });
    let material2 = material::new(Vec3(0.4, 0.2, 0.1), MaterialType::Lambertian);
    let material3 = material::new(Vec3(0.7, 0.6, 0.5), MaterialType::Metal { fuzz: 0.0 });

    let mut scene = Scene {
        objects: vec![
            shape::new(
                Vec3(0.0, 0.0, 0.0),
                ObjectType::Plane {
                    distance: (0.0),
                    normal: (Vec3::up()),
                },
                &ground_material,
            ),
            shape::new(
                Vec3(0.0, 1.0, 0.0),
                ObjectType::Sphere { radius: (1.0) },
                &material1,
            ),
            shape::new(
                Vec3(-4.0, 1.0, 0.0),
                ObjectType::Sphere { radius: (1.0) },
                &material2,
            ),
            shape::new(
                Vec3(4.0, 1.0, 0.0),
                ObjectType::Sphere { radius: (1.0) },
                &material3,
            ),
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

                    let mat = material::new(albedo, MaterialType::Lambertian);
                    scene.objects.push(shape::new(
                        center,
                        ObjectType::Sphere { radius: (0.2) },
                        &mat,
                    ));
                } else if rand_mat < 0.95 {
                    // metal
                    let albedo: Vec3 = Vec3(
                        rngs.gen_range(0.5, 1.0),
                        rngs.gen_range(0.5, 1.0),
                        rngs.gen_range(0.5, 1.0),
                    );
                    let fuzz = rngs.gen_range(0.0, 0.5);
                    let mat2 = material::new(albedo, MaterialType::Metal { fuzz });

                    scene.objects.push(shape::new(
                        center,
                        ObjectType::Sphere { radius: (0.2) },
                        &mat2,
                    ));
                } else {
                    let mat3 =
                        material::new(Vec3::zero(), MaterialType::Dielectric { refract: 1.5 });

                    scene.objects.push(shape::new(
                        center,
                        ObjectType::Sphere { radius: (0.2) },
                        &mat3,
                    ));
                }
            }
        }
    }

    scene
}
