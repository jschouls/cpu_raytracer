pub mod matrix;
pub mod vector;

use vector::{Vec3, Vec4};

pub fn schlick(cosine: f64, idx: f64) -> f64 {
    let r0 = (1.0 - idx) / (1.0 + idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
