use crate::math::schlick;
use crate::ray::Ray;
use crate::Vec3;

extern crate rand;
use rand::Rng;

use std::sync::Arc;

pub struct Material {
    albedo: Vec3, // Common
    material_type: MaterialType,
}

pub enum MaterialType {
    Lambertian,
    Metal { fuzz: f64 },
    Dielectric { refract: f64 },
}

// Because material are often created once but used for multiple objects, retuning Arc<>
pub fn new(albedo: Vec3, material_type: MaterialType) -> Arc<Material> {
    Arc::new(Material {
        albedo,
        material_type,
    })
}

pub fn scatter(material: &Material, ray_in: &Ray) -> Option<(Vec3, Ray)> {
    match &material.material_type {
        &MaterialType::Lambertian => {
            if let Some(hit) = &ray_in.is_intersected {
                let target = hit.normal + Vec3::rand_unit_vector();

                //
                return Some((material.albedo, Ray::new(hit.position, target)));
            }
            None
        }
        &MaterialType::Metal { fuzz } => {
            if let Some(hit) = &ray_in.is_intersected {
                let target = Vec3::reflect(Vec3::normalize(ray_in.direction), hit.normal);
                let scattered_ray =
                    Ray::new(hit.position, target + Vec3::rand_in_unit_sphere() * fuzz);

                if Vec3::dot(scattered_ray.direction, hit.normal) > 0.0 {
                    return Some((material.albedo, scattered_ray));
                } else {
                    return None;
                }
            }
            None
        }
        &MaterialType::Dielectric { refract } => {
            let mut rng = rand::thread_rng();
            let attenuation = Vec3::fill(1.0);
            if let Some(hit) = &ray_in.is_intersected {
                let etai = if hit.front_face {
                    1.0 / refract
                } else {
                    refract
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
}
