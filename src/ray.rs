use super::material::Material;
use super::Vec3;
use std::rc::Rc;

pub enum IntersectData {
    None,
    Found {
        material: Rc<Material>,
        normal: Vec3,
    },
}

pub struct Ray {
    pub is_intersected: IntersectData,
    pub direction: Vec3,
    pub origin: Vec3,
    pub travel_distance: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            is_intersected: IntersectData::None,
            direction: direction,
            origin: origin,
            travel_distance: std::f32::MAX,
        }
    }

    //
    pub fn set(&mut self, origin: Vec3, direction: Vec3) {
        self.is_intersected = IntersectData::None;
        self.travel_distance = std::f32::MAX;
        self.direction = direction;
        self.origin = origin;
    }
}
