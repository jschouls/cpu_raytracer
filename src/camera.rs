use super::Mat4;
use super::Vec3;
use crate::Ray;
use std::f32;

// Ratio needs to change when resolution also change.
const CAMERA_SCREEN_SIZE_HEIGHT: f32 = 0.3;
const CAMERA_SCREEN_SIZE_WIDTH: f32 = 0.4;

/**
 * viewPlane points
 *  0-----------1
 *  |           |
 *  |     c     |
 *  |           |
 *  2-----------3
 */
#[derive(Debug)]
pub struct ViewPlane {
    pub distance: f32,
    pub p: [Vec3; 4],
}

impl ViewPlane {
    pub fn new(plane_distance: f32, center: Vec3, right: Vec3, up: Vec3) -> Self {
        ViewPlane {
            distance: plane_distance,
            p: [
                center - (right * CAMERA_SCREEN_SIZE_WIDTH) + (up * CAMERA_SCREEN_SIZE_HEIGHT),
                center + (right * CAMERA_SCREEN_SIZE_WIDTH) + (up * CAMERA_SCREEN_SIZE_HEIGHT),
                center + (right * CAMERA_SCREEN_SIZE_WIDTH) - (up * CAMERA_SCREEN_SIZE_HEIGHT),
                center - (right * CAMERA_SCREEN_SIZE_WIDTH) - (up * CAMERA_SCREEN_SIZE_HEIGHT),
            ],
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub right: Vec3,
    pub vp: ViewPlane,

    //
    pub matrix: Mat4,
}

impl Camera {
    pub fn set(_pos: Vec3, _dir: Vec3, _plane_distance: f32) -> Self {
        let world_up = Vec3(0.0, 1.0, 0.0);
        // Relative to direction (right hand side)

        let lookat_matrix = Mat4::look_at(_pos, _dir, world_up);

        /*let look_direction = Vec3::normalize(_dir - _pos);

        let right = Vec3::cross(look_direction, world_up);
        // let right = look_direction.cross(world_up);
        //println!("right: {:?}", right);

        let up = Vec3::cross(right, look_direction);
        // let up = right.cross(look_direction);
        //println!("Up: {:?}", up);

        let center: Vec3 = _pos + (look_direction * _plane_distance);*/

        // Get vectors from matrix, converted from
        let right = Vec3::from(lookat_matrix.0);
        let up = Vec3::from(lookat_matrix.1);
        let forward = Vec3::from(lookat_matrix.2);

        let direction = Vec3::normalize(_dir - _pos);

        let center: Vec3 = _pos + (direction * _plane_distance);

        Camera {
            matrix: lookat_matrix,

            position: _pos,
            right: right,
            direction: direction,
            vp: ViewPlane::new(_plane_distance, center, right, up),
        }
    }

    pub fn move_direction(&mut self, delta: Vec3) {
        self.position += delta;
        self.direction = Vec3::normalize(self.direction - self.position);
        //self.direction = Vec3::normalize(self.direction);
        self.update();
    }

    pub fn update(&mut self) {
        let right = Vec3::cross(self.direction, Vec3::up());
        let up = Vec3::cross(right, self.direction);

        let center = self.position + (self.direction * self.vp.distance);
        self.right = Vec3::cross(self.direction, up);

        self.vp = ViewPlane::new(self.vp.distance, center, self.right, up)
    }

    pub fn generate_ray(&self, x: f32, y: f32) -> Ray {
        let fx = x / super::SCREEN_WIDTH as f32;
        let fy = y / super::SCREEN_HEIGHT as f32;

        // direction to viewplane
        let vp_point = self.vp.p[0]
            + ((self.vp.p[1] - self.vp.p[0]) * fx)
            + ((self.vp.p[3] - self.vp.p[0]) * fy);

        Ray::new(self.position, Vec3::normalize(vp_point - self.position))
    }
}
