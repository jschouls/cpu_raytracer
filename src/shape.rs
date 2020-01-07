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
    material: Rc<Material>, // Reference count
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

// Sphere

pub struct Sphere {
    position: Vec3,
    radius: f32,
    material: Rc<Material>,
}

impl Sphere {
    pub fn new(_position: Vec3, _radius: f32, _material: &Rc<Material>) -> Self {
        Sphere {
            position: _position,
            radius: _radius,
            material: Rc::clone(&_material),
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &mut Ray) {
        let mut _t: f32 = f32::MAX;
        let _a: f32 = Vec3::dot(ray.direction, ray.direction);
        let _b: f32 = 2.0 * Vec3::dot(ray.direction, ray.origin - self.position);
        let _c: f32 = Vec3::dot(self.position, self.position) + Vec3::dot(ray.origin, ray.origin)
            - 2.0 * Vec3::dot(self.position, ray.origin)
            - self.radius * self.radius;
        let _d: f32 = _b * _b - 4.0 * _a * _c;
        if _d < 0.0 {
            return;
        }

        let _t1 = (-_b - _d.sqrt()) / (2.0 * _a);
        let _t2 = (-_b + _d.sqrt()) / (2.0 * _a);

        if (_t1 < _t) && (_t1 >= f32::EPSILON) {
            _t = _t1;
        }
        if (_t2 < _t) && (_t2 >= f32::EPSILON) {
            _t = _t2;
        }
        if _t < ray.travel_distance {
            let p_i = ray.origin + ray.direction * _t;
            let _normal = Vec3::normalize(p_i - self.position);
            ray.is_intersected = IntersectData::Found {
                material: Rc::clone(&self.material),
                normal: _normal,
            };
            ray.travel_distance = _t;
        }
    }
}
