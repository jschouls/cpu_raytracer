use super::material::Material;
use crate::math::vector::Vec3;
use crate::ray::Ray;

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray) -> Option<(Vec3, Ray)> {
        if let Some(hit) = &ray_in.is_intersected {
            let target = Vec3::reflect(Vec3::normalize(ray_in.direction), hit.normal);
            let scattered_ray = Ray::new(
                hit.position,
                target + Vec3::rand_in_unit_sphere() * self.fuzz,
            );

            if Vec3::dot(scattered_ray.direction, hit.normal) > 0.0 {
                return Some((self.albedo, scattered_ray));
            } else {
                return None;
            }
        }
        None
    }
}
