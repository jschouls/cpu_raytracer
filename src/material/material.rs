use crate::ray::Ray;
use crate::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, attenuation: Vec3) -> Option<Ray>;
}
