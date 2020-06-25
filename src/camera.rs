use super::ray::Ray;
use super::Vec3;
use std::f64;

// Ratio needs to change when resolution also change.
const CAMERA_SCREEN_SIZE_HEIGHT: f64 = 0.3;
const CAMERA_SCREEN_SIZE_WIDTH: f64 = 0.4;

/**
 * viewPlane points
 *  0-----------1
 *  |           |
 *  |     c     |
 *  |           |
 *  2-----------3
 */

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub lower_top_corner: Vec3,

    pub horizonal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    // Note: because I draw from top left,
    pub fn set(position: Vec3, look_at: Vec3, up: Vec3, fov: f64, ratio: f64) -> Camera {
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let view_height = 2.0 * h;
        let view_width = ratio * view_height;

        let to = Vec3::normalize(position - look_at);
        let u = Vec3::normalize(Vec3::cross(up, to));
        let v = Vec3::cross(to, u);

        let horizonal = u * view_width;
        let vertical = v * view_height;
        // + vertical because I draw from top left
        let lt_corner = position - (horizonal / 2.0) + (vertical / 2.0) - to;

        Camera {
            position,
            direction: to,
            horizonal,
            vertical,
            lower_top_corner: lt_corner,
        }
    }

    pub fn generate_ray(&self, x: f64, y: f64) -> Ray {
        let fx = x / super::SCREEN_WIDTH as f64;
        let fy = y / super::SCREEN_HEIGHT as f64;
        Ray::new(
            self.position,
            self.lower_top_corner + (self.horizonal * fx) - (self.vertical * fy) - self.position,
        )
    }
}
