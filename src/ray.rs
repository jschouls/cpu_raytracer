use super::material::Material;
use super::vector::Vec3;
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
