use super::Vec3;
#[derive(Clone)]
pub enum LightType {
    /*Point,

    Area {}
    */
}
#[derive(Clone)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
    pub light_type: LightType,
}
