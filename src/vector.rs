use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec2(pub f32, pub f32);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

trait Vector {
    fn squared(&self) -> f32;
    fn length(&self) -> f32 {
        self.squared().sqrt()
    }

    fn normalize(&self) -> Self;
    fn dot(&self, other: Self) -> f32;
}

/***
 *  Vector 2 implementations
***/
impl Vec2 {
    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn zero() -> Self {
        Self(0.0, 0.0)
    }

    pub fn up() -> Self {
        Self(0.0, 1.0)
    }
}

impl Vector for Vec2 {
    fn squared(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1)
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        Self(self.0 / length, self.1 / length)
    }

    fn dot(&self, other: Self) -> f32 {
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
impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
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
    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn cross(&self, other: Vec3) -> Self {
        Self(
            (self.1 * other.2) - (self.2 * other.1),
            (self.2 * other.0) - (self.0 * other.2),
            (self.0 * other.1) - (self.1 * other.0),
        )
    }

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
}

impl Vector for Vec3 {
    fn squared(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        Self(self.0 / length, self.1 / length, self.2 / length)
    }

    fn dot(&self, other: Self) -> f32 {
        (self.0 * other.0 + self.1 * other.1 + self.2 * other.2)
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
impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
        Self {
            0: self.0 * _rhs,
            1: self.1 * _rhs,
            2: self.2 * _rhs,
        }
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

    fn test_vector3_cross() {
        let a = Vec3(2.0, 3.0, 4.0);
        let b = Vec3(5.0, 6.0, 7.0);
        let cross = a.cross(b);

        assert_eq!(cross, Vec3(-3.0, 6.0, -3.0))
    }
}
