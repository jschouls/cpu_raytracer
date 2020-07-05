use super::material::Material;
use super::Vec3;
use std::sync::Arc;

pub struct IntersectData {
    pub material: Arc<dyn Material>,
    pub position: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

pub struct Ray {
    pub is_intersected: Option<IntersectData>,
    pub direction: Vec3,
    pub origin: Vec3,
    pub travel_distance: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            is_intersected: None,
            direction,
            origin,
            travel_distance: std::f64::MAX,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }

    //pub fn set_intersection(&mut self, ray: &Ray, mat: Rc<Material>, normal: Vec3) {
    pub fn set_intersection(&mut self, t: f64, material: Arc<dyn Material>, normal: Vec3) {
        let _is_inside = Vec3::dot(self.direction, normal) < 0.0;
        self.travel_distance = t;
        self.is_intersected = Some(IntersectData {
            material,
            position: self.at(t),
            front_face: _is_inside,
            normal: if _is_inside { normal } else { -normal },
        });
    }
}
