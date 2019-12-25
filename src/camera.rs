use crate::vector::Vec3;

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

        let right = _dir.cross(world_up);
        println!("right: {:?}", right);

        let up = right.cross(_dir);
        println!("Up: {:?}", up);

        let center: Vec3 = _pos + (_dir * _plane_distance);

        Camera {
            position: _pos,
            direction: _dir,
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
}
