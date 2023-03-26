use super::ray::Ray;
use super::Vec3;
use std::f64;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub lower_top_corner: Vec3,

    pub horizonal: Vec3,
    pub vertical: Vec3,

    pub u: Vec3,
    pub v: Vec3,

    pub lens_radius: f64,
}

impl Camera {
    pub fn set(
        position: Vec3,
        look_at: Vec3,
        up: Vec3,
        fov: f64,
        ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let view_height = 2.0 * h;
        let view_width = ratio * view_height;

        let to = Vec3::normalize(position - look_at);
        let u = Vec3::normalize(Vec3::cross(up, to));
        let v = Vec3::cross(to, u);

        let horizonal = u * view_width * focus_dist;
        let vertical = v * view_height * focus_dist;
        // + vertical because I draw from top left
        let lt_corner = position - (horizonal / 2.0) + (vertical / 2.0) - to * focus_dist;

        Camera {
            position,
            direction: to,
            horizonal,
            vertical,
            u,
            v,
            lower_top_corner: lt_corner,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn generate_ray(&self, x: f64, y: f64) -> Ray {
        let rd = Vec3::rand_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.0 + self.v * rd.1;

        let fx = x / super::SCREEN_WIDTH as f64;
        let fy = y / super::SCREEN_HEIGHT as f64;
        Ray::new(
            self.position + offset,
            self.lower_top_corner + (self.horizonal * fx)
                - (self.vertical * fy)
                - self.position
                - offset,
        )
    }
}
