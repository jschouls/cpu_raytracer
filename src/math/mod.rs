pub mod matrix;
pub mod vector;

use vector::{Vec3, Vec4};

// because std lib clamp is unstable
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}
