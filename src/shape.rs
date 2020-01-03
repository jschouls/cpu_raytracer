use super::material::Material;
use super::ray::IntersectData;
use super::ray::Ray;
use super::vector::Vec3;
use sdl2::pixels;
use std::f32;
use std::rc::Rc;

pub trait Shape {
    fn intersect(&self, ray: &mut Ray);

    // Default impl
    fn get_color(&self, point_intersect: Vec3) -> pixels::Color {
        // Default black color
        pixels::Color::RGB(0, 0, 0)
    }
}

// Infinite plane
pub struct Plane {
    normal: Vec3,
    distance: f32,
    material: Rc<Material>,
}

impl Plane {
    pub fn new(_normal: Vec3, _distance: f32, _material: &Rc<Material>) -> Self {
        Plane {
            normal: _normal,
            distance: _distance,
            material: Rc::clone(&_material),
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &mut Ray) {
        let t = -(Vec3::dot(ray.origin, self.normal) + self.distance)
            / Vec3::dot(ray.direction, self.normal);

        if t < ray.travel_distance && t >= f32::EPSILON {
            ray.is_intersected = IntersectData::Found {
                material: Rc::clone(&self.material),
                normal: self.normal,
            };
            ray.travel_distance = t;
        }
    }

    fn get_color(&self, point_intersect: Vec3) -> pixels::Color {
        pixels::Color::RGB(0, 0, 0)
    }
}

pub struct Sphere {
    position: Vec3,
    radius: f32,
    material: Material,
}
