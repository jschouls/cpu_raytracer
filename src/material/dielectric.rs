use super::material::Material;
use crate::math::schlick;
use crate::math::vector::Vec3;
use crate::ray::Ray;

extern crate rand;
use rand::prelude::*;
pub struct Dielectric {
    pub refract: f64,
}

impl Dielectric {
    pub fn new(refract: f64) -> Dielectric {
        Dielectric { refract: refract }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray) -> Option<(Vec3, Ray)> {
        let mut rng = rand::thread_rng();
        let attenuation = Vec3::fill(1.0);
        if let Some(hit) = &ray_in.is_intersected {
            let etai = if hit.front_face {
                1.0 / self.refract
            } else {
                self.refract
            };

            let unit_direction = Vec3::normalize(ray_in.direction);

            let cos_theta = Vec3::dot(-unit_direction, hit.normal).min(1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
            if etai * sin_theta > 1.0 {
                let reflect = Vec3::reflect(unit_direction, hit.normal);
                return Some((attenuation, Ray::new(hit.position, reflect)));
            }
            let reflect_prob = schlick(cos_theta, etai);
            let rand: f64 = rng.gen();
            if rand < reflect_prob {
                let reflect = Vec3::reflect(unit_direction, hit.normal);
                return Some((attenuation, Ray::new(hit.position, reflect)));
            }

            let refracted = Vec3::refract(unit_direction, hit.normal, etai);
            return Some((attenuation, Ray::new(hit.position, refracted)));
        }
        None
    }
}
