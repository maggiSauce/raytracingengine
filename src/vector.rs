#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn add(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z,
    }
}

pub fn subtract(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

pub fn scalar_multiply(s: f64, v: Vec3) -> Vec3 {
    Vec3 {
        x: s * v.x,
        y: s * v.y,
        z: s * v.z,
    }
}

pub fn scalar_divide(v: Vec3, d:f64) -> Vec3 {
    scalar_multiply(1.0/d, v)
}

pub fn length2(v: Vec3) -> f64 {
    (v.x * v.x) + (v.y * v.y) + (v.z * v.z)
}

pub fn length(v: Vec3) -> f64 {
    length2(v).sqrt()
}

pub fn normalize(v: Vec3) -> Vec3 {
    scalar_divide(v, length(v))
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
}

pub fn distance2(v1: Vec3, v2: Vec3) -> f64 {
    length2(subtract(v1, v2))
}

pub fn distance(v1: Vec3, v2: Vec3) -> f64 {
    length(subtract(v1, v2))
}

//=======================================================
// Tests
//=======================================================


#[cfg(test)]
mod tests {
    use super::*;

    // A tiny threshold to handle floating-point precision differences
    const EPSILON: f64 = 1e-10;

    fn approx_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    fn vec_approx_equal(v1: Vec3, v2: Vec3) -> bool {
        approx_equal(v1.x, v2.x) && approx_equal(v1.y, v2.y) && approx_equal(v1.z, v2.z)
    }

    #[test]
    fn test_add() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let result = add(v1, v2);
        assert!(vec_approx_equal(result, Vec3 { x: 5.0, y: 7.0, z: 9.0 }));
    }

    #[test]
    fn test_subtract() {
        let v1 = Vec3 { x: 5.0, y: 7.0, z: 9.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let result = subtract(v1, v2);
        assert!(vec_approx_equal(result, Vec3 { x: 1.0, y: 2.0, z: 3.0 }));
    }

    #[test]
    fn test_scalar_multiply() {
        let v = Vec3 { x: 1.0, y: -2.0, z: 3.5 };
        let result = scalar_multiply(2.0, v);
        assert!(vec_approx_equal(result, Vec3 { x: 2.0, y: -4.0, z: 7.0 }));
    }

    #[test]
    fn test_scalar_divide() {
        let v = Vec3 { x: 2.0, y: -4.0, z: 8.0 };
        let result = scalar_divide(v, 2.0);
        assert!(vec_approx_equal(result, Vec3 { x: 1.0, y: -2.0, z: 4.0 }));
    }

    #[test]
    fn test_length2() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 2.0 };
        // 1^2 + 2^2 + 2^2 = 1 + 4 + 4 = 9
        assert!(approx_equal(length2(v), 9.0));
    }

    #[test]
    fn test_length() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 2.0 };
        // sqrt(9) = 3
        assert!(approx_equal(length(v), 3.0));
    }

    #[test]
    fn test_normalize() {
        let v = Vec3 { x: 3.0, y: 0.0, z: 0.0 };
        let result = normalize(v);
        assert!(vec_approx_equal(result, Vec3 { x: 1.0, y: 0.0, z: 0.0 }));

        // A normalized vector should always have a length of 1.0
        let v2 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert!(approx_equal(length(normalize(v2)), 1.0));
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3 { x: 1.0, y: 3.0, z: -5.0 };
        let v2 = Vec3 { x: 4.0, y: -2.0, z: -1.0 };
        // (1*4) + (3*-2) + (-5*-1) = 4 - 6 + 5 = 3
        assert!(approx_equal(dot(v1, v2), 3.0));
    }

    #[test]
    fn test_distance2() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 1.0, y: 5.0, z: 7.0 };
        // delta components: x=0, y=3, z=4
        // 0^2 + 3^2 + 4^2 = 0 + 9 + 16 = 25
        assert!(approx_equal(distance2(v1, v2), 25.0));
    }

    #[test]
    fn test_distance() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 1.0, y: 5.0, z: 7.0 };
        // sqrt(25) = 5
        assert!(approx_equal(distance(v1, v2), 5.0));
    }
}