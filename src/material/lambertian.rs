use super::material::Material;
use crate::math::vector::Vec3;
use crate::ray::Ray;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray) -> Option<(Vec3, Ray)> {
        if let Some(hit) = &ray_in.is_intersected {
            let target = hit.normal + Vec3::rand_unit_vector();

            //
            return Some((self.albedo, Ray::new(hit.position, target)));
        }
        None
    }
}
