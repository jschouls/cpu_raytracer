use super::Vec3;
use super::Vec4;
#[derive(Debug)]
pub struct Mat4(pub Vec4, pub Vec4, pub Vec4, pub Vec4);

impl Mat4 {
    pub fn look_at(position: Vec3, to: Vec3, up: Vec3) -> Self {
        let forward = Vec3::normalize(position - to);
        let right = Vec3::cross(up, forward);
        let up = Vec3::cross(forward, right);

        Self(
            Vec4(right.0, right.1, right.2, 0.0),
            Vec4(up.0, up.1, up.2, 0.0),
            Vec4(forward.0, forward.1, forward.2, 0.0),
            Vec4(position.0, position.1, position.2, 1.0),
        )
    }
}
