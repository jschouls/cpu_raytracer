use super::light::{Light, LightType};
use super::shape::{Plane, Shape, Sphere};
use super::Material;
use super::Vec3;

use std::rc::Rc;

pub struct Scene {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>,
}

// Create the scene.
pub fn create_scene() -> Scene {
    // Materials

    // Reference counter because this can be shared with others and rays.
    let floor_material = Rc::new(Material {
        color: sdl2::pixels::Color::RGB(255, 255, 255),
        reflection: 0.0,
        refraction: 0.0,
    });

    let back_material = Rc::new(Material {
        color: sdl2::pixels::Color::RGB(255, 0, 0),
        reflection: 0.0,
        refraction: 0.0,
    });

    Scene {
        objects: vec![
            Box::new(Plane::new(Vec3::up(), 2.5, &floor_material)),
            Box::new(Plane::new(Vec3(0.0, 0.0, 1.0), 5.0, &back_material)),
            Box::new(Plane::new(Vec3(1.0, 0.0, 0.0), 5.0, &back_material)),
            Box::new(Plane::new(Vec3(-1.0, 0.0, 0.0), 5.0, &back_material)),
            Box::new(Sphere::new(Vec3(2.0, 0.5, -4.0), 1.0, &floor_material)),
        ],
        lights: vec![Light {
            position: Vec3(0.0, 2.5, -2.5),
            intensity: 5.0,
            light_type: LightType::Point,
        }],
    }
}
