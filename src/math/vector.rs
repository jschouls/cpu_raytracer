use sdl2::pixels::Color;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec2(pub f64, pub f64);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec4(pub f64, pub f64, pub f64, pub f64);

pub trait Vector {
    fn squared(&self) -> f64;
    fn length(&self) -> f64 {
        self.squared().sqrt()
    }

    fn normalize(&self) -> Self;
    fn dot(&self, other: Self) -> f64;
}

/***
 *  Vector 2 implementations
***/
impl Vec2 {
    //fn to_vec3(&self) -> Vec3 {
    //    Vec3(self.0, self.1, 0.0)
    //}
}

impl Vector for Vec2 {
    fn squared(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1)
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        Self(self.0 / length, self.1 / length)
    }

    fn dot(&self, other: Self) -> f64 {
        (self.0 * other.0 + self.1 * other.1)
    }
}

// Vector2 operators overload

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            0: self.0 + _rhs.0,
            1: self.1 + _rhs.1,
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            0: self.0 - _rhs.0,
            1: self.1 - _rhs.1,
        }
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self {
        Self {
            0: self.0 * _rhs.0,
            1: self.1 * _rhs.1,
        }
    }
}
// scalar mul
impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        Self {
            0: self.0 * _rhs,
            1: self.1 * _rhs,
        }
    }
}

/***
 *  Vector 3 implementations
***/

impl Vec3 {
    pub fn to_color(&self) -> Color {
        let _r = clamp(self.0 * 225.0, 0.0, 255.0);
        let _g = clamp(self.1 * 225.0, 0.0, 255.0);
        let _b = clamp(self.2 * 225.0, 0.0, 255.0);
        Color::RGB(_r as u8, _g as u8, _b as u8)
    }

    pub fn from_color(color: Color) -> Self {
        Vec3(color.r as f64, color.g as f64, color.b as f64)
    }

    // pub fn cross(&self, other: Vec3) -> Self {
    //     Self(
    //         (self.1 * other.2) - (self.2 * other.1),
    //         (self.2 * other.0) - (self.0 * other.2),
    //         (self.0 * other.1) - (self.1 * other.0),
    //     )
    // }

    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn up() -> Self {
        Self(0.0, 1.0, 0.0)
    }

    // could not call from trait?
    pub fn normalize(v: Self) -> Self {
        v.normalize()
    }

    pub fn dot(v1: Self, v2: Self) -> f64 {
        (v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2)
    }

    pub fn cross(v1: Self, v2: Self) -> Self {
        Self(
            (v1.1 * v2.2) - (v1.2 * v2.1),
            (v1.2 * v2.0) - (v1.0 * v2.2),
            (v1.0 * v2.1) - (v1.1 * v2.0),
        )
    }
}

impl Vector for Vec3 {
    fn squared(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
    }

    fn length(&self) -> f64 {
        self.squared().sqrt()
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        Self(self.0 / length, self.1 / length, self.2 / length)
    }

    fn dot(&self, other: Self) -> f64 {
        (self.0 * other.0 + self.1 * other.1 + self.2 * other.2)
    }
}

// conversions
impl From<Vec4> for Vec3 {
    fn from(vec: Vec4) -> Self {
        Vec3(vec.0, vec.1, vec.2)
    }
}

impl From<Vec3> for Vec4 {
    fn from(vec: Vec3) -> Self {
        Vec4(vec.0, vec.1, vec.2, 0.0)
    }
}

// Vector3 operators overload

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            0: self.0 + _rhs.0,
            1: self.1 + _rhs.1,
            2: self.2 + _rhs.2,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Self) {
        *self = Self {
            0: self.0 + _rhs.0,
            1: self.1 + _rhs.1,
            2: self.2 + _rhs.2,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            0: self.0 - _rhs.0,
            1: self.1 - _rhs.1,
            2: self.2 - _rhs.2,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self {
        Self {
            0: self.0 * _rhs.0,
            1: self.1 * _rhs.1,
            2: self.2 * _rhs.2,
        }
    }
}

// scalar mul
impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        Self {
            0: self.0 * _rhs,
            1: self.1 * _rhs,
            2: self.2 * _rhs,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self::Output {
        Self {
            0: self.0 / _rhs,
            1: self.1 / _rhs,
            2: self.2 / _rhs,
        }
    }
}

// Helper functions
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}

/***
 *  Tests
***/

#[cfg(test)]
mod unit_tests {

    use super::*;

    // Vec 2
    #[test]
    fn test_vector2_add() {
        assert_eq!(Vec2(5.0, 5.0) + Vec2(5.0, 5.0), Vec2(10.0, 10.0));
    }

    #[test]
    fn test_vector2_sub() {
        assert_eq!(Vec2(5.0, 5.0) - Vec2(5.0, 5.0), Vec2(0.0, 0.0));
    }

    #[test]
    fn test_vector2_mul() {
        assert_eq!(Vec2(5.0, 5.0) * Vec2(5.0, 5.0), Vec2(25.0, 25.0));
    }

    #[test]
    fn test_vector2_mul_scalar() {
        assert_eq!(Vec2(5.0, 5.0) * 10.0, Vec2(50.0, 50.0));
    }

    #[test]
    fn test_vector2_squared() {
        let v = Vec2(3.0, 2.0);
        assert_eq!(v.squared(), (3.0 * 3.0 + 2.0 * 2.0));
    }

    #[test]
    fn test_vector2_length() {
        let v = Vec2(3.0, 2.0);

        assert_eq!(v.length(), 3.6055512);
    }

    #[test]
    fn test_vector2_normalize() {
        let v = Vec2(3.0, 2.0);
        assert_eq!(v.length(), 3.6055512);
        let v_normalized = v.normalize();
        assert_eq!(v_normalized.length(), 1.0);
    }

    // Vec 3

    #[test]

    fn test_vector3_add() {
        assert_eq!(
            Vec3(5.0, 5.0, 5.0) + Vec3(5.0, 5.0, 5.0),
            Vec3(10.0, 10.0, 10.0)
        );
    }

    #[test]
    fn test_vector3_sub() {
        assert_eq!(
            Vec3(5.0, 5.0, 5.0) - Vec3(5.0, 5.0, 5.0),
            Vec3(0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn test_vector3_mul() {
        assert_eq!(
            Vec3(5.0, 5.0, 5.0) * Vec3(5.0, 5.0, 5.0),
            Vec3(25.0, 25.0, 25.0)
        );
    }

    #[test]
    fn test_vector3_mul_scalar() {
        assert_eq!(Vec3(5.0, 5.0, 5.0) * 10.0, Vec3(50.0, 50.0, 50.0));
    }

    #[test]
    fn test_vector3_squared() {
        let v = Vec3(3.0, 2.0, 5.0);
        assert_eq!(v.squared(), (3.0 * 3.0 + 2.0 * 2.0 + 5.0 * 5.0));
    }

    #[test]
    fn test_vector3_length() {
        let v = Vec3(3.0, 2.0, 5.0);

        assert_eq!(v.length(), 6.16441400);
    }

    #[test]
    fn test_vector3_normalize() {
        let v = Vec3(3.0, 2.0, 5.0);
        assert_eq!(v.length(), 6.16441400);
        let v_normalized = v.normalize();
        assert_eq!(v_normalized.length(), 1.0);
    }

    #[test]
    fn test_vector3_cross() {
        let a = Vec3(2.0, 3.0, 4.0);
        let b = Vec3(5.0, 6.0, 7.0);
        let cross = Vec3::cross(a, b);

        assert_eq!(cross, Vec3(-3.0, 6.0, -3.0))
    }
}
