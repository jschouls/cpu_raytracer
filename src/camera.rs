use crate::vector::Vec3;
use crate::Ray;
use std::f32;

const CAMERA_SCREEN_SIZE: f32 = 0.5;

/**
 * viewPlane points
 *  0-----------1
 *  |           |
 *  |           |
 *  |           |
 *  2-----------3
 */
#[derive(Debug)]
pub struct ViewPlane {
    pub distance: f32,
    pub p: [Vec3; 4],
}

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub vp: ViewPlane,
}

impl Camera {
    pub fn set(_pos: Vec3, _dir: Vec3, _plane_distance: f32) -> Self {
        let world_up = Vec3(0.0, 1.0, 0.0);
        // Relative to direction (right hand side)

        let look_direction = Vec3::normalize(_dir - _pos);

        let right = _dir.cross(world_up);
        //println!("right: {:?}", right);

        let up = right.cross(look_direction);
        //println!("Up: {:?}", up);

        let center: Vec3 = _pos + (look_direction * _plane_distance);

        Camera {
            position: _pos,
            direction: look_direction,
            vp: ViewPlane {
                distance: _plane_distance,
                p: [
                    center - (right * CAMERA_SCREEN_SIZE) + (up * CAMERA_SCREEN_SIZE),
                    center + (right * CAMERA_SCREEN_SIZE) + (up * CAMERA_SCREEN_SIZE),
                    center + (right * CAMERA_SCREEN_SIZE) - (up * CAMERA_SCREEN_SIZE),
                    center - (right * CAMERA_SCREEN_SIZE) - (up * CAMERA_SCREEN_SIZE),
                ],
            },
        }
    }

    pub fn generate_ray(&self, x: f32, y: f32) -> Ray {
        let fx = x / super::SCREEN_WIDTH as f32;
        let fy = y / super::SCREEN_HEIGHT as f32;

        // direction to viewplane
        let vp_point = self.vp.p[0]
            + ((self.vp.p[1] - self.vp.p[0]) * fx)
            + ((self.vp.p[3] - self.vp.p[0]) * fy);

        //let x = (vp_point - self.position);
        //x.normalize();

        Ray {
            is_intersected: super::IntersectData::None,
            origin: self.position,
            direction: Vec3::normalize(vp_point - self.position),
            travel_distance: f32::MAX,
        }
    }
}
