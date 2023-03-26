use super::material::Material;
use super::ray::Ray;
use super::Vec3;

use std::f64;
use std::sync::Arc;

//
#[derive(Clone)]
pub struct Object {
    position: Vec3,
    object_type: ObjectType,
    material: Arc<Material>, // TODO: refactor
}
#[derive(Clone)]
pub enum ObjectType {
    Sphere { radius: f64 },
    Plane { distance: f64, normal: Vec3 },
}

pub fn new(position: Vec3, object_type: ObjectType, material: &Arc<Material>) -> Object {
    Object {
        position,
        object_type,
        material: Arc::clone(&material),
    }
}

//
pub fn intersect(obj: &Object, ray: &mut Ray, tolerance: f64) {
    match &obj.object_type {
        // Intersect for sphere
        &ObjectType::Sphere { radius } => {
            let mut _t: f64 = f64::MAX;
            let _a: f64 = Vec3::dot(ray.direction, ray.direction);
            let _b: f64 = 2.0 * Vec3::dot(ray.direction, ray.origin - obj.position);
            let _c: f64 = Vec3::dot(obj.position, obj.position) + Vec3::dot(ray.origin, ray.origin)
                - 2.0 * Vec3::dot(obj.position, ray.origin)
                - radius * radius;
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
                let _normal = (point_intersect - obj.position) / radius;
                ray.set_intersection(_t, Arc::clone(&obj.material), _normal);
            }
        }
        // Intersect for plane
        &ObjectType::Plane { distance, normal } => {
            let t = -(Vec3::dot(ray.origin, normal) + distance) / Vec3::dot(ray.direction, normal);

            if t < ray.travel_distance && t >= tolerance {
                ray.set_intersection(t, Arc::clone(&obj.material), normal);
            }
        }
    }
}
