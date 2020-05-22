use super::Vec3;

pub enum LightType {
    /*Point,

Area {}
*/}

pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
    pub light_type: LightType,
}
