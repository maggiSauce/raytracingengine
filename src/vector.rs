use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }    
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, s: f64) -> Vec3 {
        Vec3 {
            x: s * self.x,
            y: s * self.y,
            z: s * self.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, d:f64) -> Vec3 {
        self * (1.0/d)
    }
}

impl Vec3 {
    pub fn length2(self: Vec3) -> f64 {
    (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(self) -> f64 {
        self.length2().sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }
}


pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
}

pub fn distance2(v1: Vec3, v2: Vec3) -> f64 {
    (v1 - v2).length2()
}

pub fn distance(v1: Vec3, v2: Vec3) -> f64 {
    (v1 - v2).length()
}

//=======================================================
// Tests
//=======================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) {
        assert!(
            (a - b).abs() < EPSILON,
            "Expected {}, got {}",
            b,
            a
        );
    }

    fn assert_vec3_eq(v: Vec3, x: f64, y: f64, z: f64) {
        approx_eq(v.x, x);
        approx_eq(v.y, y);
        approx_eq(v.z, z);
    }

    #[test]
    fn add_vectors() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

        let c = a + b;

        assert_vec3_eq(c, 5.0, 7.0, 9.0);
    }

    #[test]
    fn subtract_vectors() {
        let a = Vec3 { x: 5.0, y: 7.0, z: 9.0 };
        let b = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

        let c = a - b;

        assert_vec3_eq(c, 4.0, 5.0, 6.0);
    }

    #[test]
    fn multiply_by_scalar() {
        let v = Vec3 { x: 1.0, y: -2.0, z: 3.0 };

        let r = v * 2.5;

        assert_vec3_eq(r, 2.5, -5.0, 7.5);
    }

    #[test]
    fn divide_by_scalar() {
        let v = Vec3 { x: 2.0, y: -4.0, z: 6.0 };

        let r = v / 2.0;

        assert_vec3_eq(r, 1.0, -2.0, 3.0);
    }

    #[test]
    fn length_squared() {
        let v = Vec3 { x: 2.0, y: 3.0, z: 6.0 };

        approx_eq(v.length2(), 49.0);
    }

    #[test]
    fn length() {
        let v = Vec3 { x: 3.0, y: 4.0, z: 12.0 };

        approx_eq(v.length(), 13.0);
    }

    #[test]
    fn normalize_vector() {
        let v = Vec3 { x: 3.0, y: 0.0, z: 4.0 };

        let n = v.normalize();

        approx_eq(n.length(), 1.0);
        assert_vec3_eq(n, 0.6, 0.0, 0.8);
    }

    #[test]
    fn dot_product() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: -5.0, z: 6.0 };

        approx_eq(dot(a, b), 12.0);
    }

    #[test]
    fn distance_squared() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: 6.0, z: 3.0 };

        approx_eq(distance2(a, b), 25.0);
    }

    #[test]
    fn distance_test() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: 6.0, z: 3.0 };

        approx_eq(distance(a, b), 5.0);
    }

    #[test]
    fn normalize_unit_vector() {
        let v = Vec3 { x: 1.0, y: 0.0, z: 0.0 };

        let n = v.normalize();

        assert_vec3_eq(n, 1.0, 0.0, 0.0);
    }

    #[test]
    fn dot_with_self_equals_length_squared() {
        let v = Vec3 { x: 2.0, y: -3.0, z: 6.0 };

        approx_eq(dot(v, v), v.length2());
    }

    #[test]
    fn distance_to_self_is_zero() {
        let v = Vec3 { x: -5.0, y: 1.0, z: 8.0 };

        approx_eq(distance(v, v), 0.0);
        approx_eq(distance2(v, v), 0.0);
    }
}