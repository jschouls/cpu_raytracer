use super::material::Material;
use super::ray::Ray;
use super::Vec3;

use std::f64;
use std::sync::Arc;

pub trait Shape: Sync + Send {
    fn intersect(&self, ray: &mut Ray, tolerance: f64);

    // Default impl
    fn get_color(&self, _point_intersect: Vec3) -> Vec3 {
        // Default black color
        Vec3(0.0, 0.0, 0.0)
    }

    // fn draw_debug(&self, _canvas: &mut Canvas<Window>) -> Result<(), String> {
    //     Ok(())
    // }
}

// Infinite plane
pub struct Plane {
    normal: Vec3,
    distance: f64,
    material: Arc<dyn Material>, // Reference count
}

impl Plane {
    pub fn new(_normal: Vec3, _distance: f64, _material: &Arc<dyn Material>) -> Self {
        Plane {
            normal: _normal,
            distance: _distance,
            material: Arc::clone(&_material),
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &mut Ray, tolerance: f64) {
        let t = -(Vec3::dot(ray.origin, self.normal) + self.distance)
            / Vec3::dot(ray.direction, self.normal);

        if t < ray.travel_distance && t >= tolerance {
            ray.set_intersection(t, Arc::clone(&self.material), self.normal);
        }
    }

    fn get_color(&self, _point_intersect: Vec3) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}

// Sphere

pub struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(_position: Vec3, _radius: f64, _material: &Arc<dyn Material>) -> Self {
        Sphere {
            position: _position,
            radius: _radius,
            material: Arc::clone(&_material),
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &mut Ray, tolerance: f64) {
        let mut _t: f64 = f64::MAX;
        let _a: f64 = Vec3::dot(ray.direction, ray.direction);
        let _b: f64 = 2.0 * Vec3::dot(ray.direction, ray.origin - self.position);
        let _c: f64 = Vec3::dot(self.position, self.position) + Vec3::dot(ray.origin, ray.origin)
            - 2.0 * Vec3::dot(self.position, ray.origin)
            - self.radius * self.radius;
        let _d: f64 = _b * _b - 4.0 * _a * _c;
        if _d < 0.0 {
            return;
        }

        let _t1 = (-_b - _d.sqrt()) / (2.0 * _a);
        let _t2 = (-_b + _d.sqrt()) / (2.0 * _a);

        if (_t1 < _t) && (_t1 >= tolerance) {
            _t = _t1;
        }
        if (_t2 < _t) && (_t2 >= tolerance) {
            _t = _t2;
        }
        if _t < ray.travel_distance {
            let point_intersect = ray.at(_t); //ray.origin + ray.direction * _t;
            let _normal = (point_intersect - self.position) / self.radius;
            ray.set_intersection(_t, Arc::clone(&self.material), _normal);
        }
    }

    #[cfg(feature = "draw-debugger")]
    fn draw_debug(&self, _canvas: &mut Canvas<Window>) -> Result<(), String> {
        let mut p1 = Vec2(
            self.radius * 0.0 + self.position.0,
            self.radius * 1.0 + self.position.2,
        );

        let mut p2 = Vec2(
            self.radius * 0.0 + self.position.0,
            self.radius * 1.0 + self.position.2,
        );

        let res = 72;
        for i in 1..res {
            let rad = (i as f32 * 5.0) * std::f32::consts::PI / 180.0;
            p2.0 = self.radius * rad.sin() + self.position.0; // x
            p2.1 = self.radius * rad.cos() + self.position.2; // z
            scene::draw_line(_canvas, p1, p2, Color::RGB(255, 255, 255))?;
            p1 = p2;
        }

        Ok(())
    }
}
