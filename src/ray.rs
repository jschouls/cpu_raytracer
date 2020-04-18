use super::material::Material;
use super::Vec3;
use std::rc::Rc;

pub struct IntersectData {
    pub material: Rc<Material>,
    pub normal: Vec3,
    pub is_inside: bool,
}

pub struct Ray {
    pub is_intersected: Option<IntersectData>,
    pub direction: Vec3,
    pub origin: Vec3,
    pub travel_distance: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            is_intersected: None,
            direction: direction,
            origin: origin,
            travel_distance: std::f64::MAX,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn set_intersection(&mut self, t: f64, mat: Rc<Material>, normal: Vec3) {
        let _is_inside = Vec3::dot(self.direction, normal) > 0.0;
        self.is_intersected = Some(IntersectData {
            material: mat,
            is_inside: _is_inside,
            normal: if _is_inside { -normal } else { normal },
        });
        self.travel_distance = t;
    }

    //
    // pub fn set(&mut self, origin: Vec3, direction: Vec3) {
    //     self.is_intersected = IntersectData::None;
    //     self.travel_distance = std::f64::MAX;
    //     self.direction = direction;
    //     self.origin = origin;
    // }
}
