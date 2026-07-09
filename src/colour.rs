use crate::vector::*;

pub fn unpackRGB(packedRGB: u32) -> Vec3 {
    let mask:u32 = 0xFF;
    Vec3 {
        x: (((packedRGB >> 16) & mask) as f64) / 255.0,
        y: (((packedRGB >> 8) & mask) as f64) / 255.0,
        z: (((packedRGB) & mask) as f64) / 255.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------
    // helpers
    // -------------------------

    fn assert_vec3_close(actual: Vec3, expected: Vec3) {
        let epsilon = 1e-10;

        assert!(
            (actual.x - expected.x).abs() < epsilon,
            "x mismatch: got {}, expected {}",
            actual.x,
            expected.x
        );

        assert!(
            (actual.y - expected.y).abs() < epsilon,
            "y mismatch: got {}, expected {}",
            actual.y,
            expected.y
        );

        assert!(
            (actual.z - expected.z).abs() < epsilon,
            "z mismatch: got {}, expected {}",
            actual.z,
            expected.z
        );
    }

    // -------------------------
    // unpackRGB tests
    // -------------------------

    #[test]
    fn test_unpack_black() {
        let result = unpackRGB(0x000000);

        assert_vec3_close(
            result,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );
    }

    #[test]
    fn test_unpack_white() {
        let result = unpackRGB(0xFFFFFF);

        assert_vec3_close(
            result,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        );
    }

    #[test]
    fn test_unpack_red() {
        let result = unpackRGB(0xFF0000);

        assert_vec3_close(
            result,
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        );
    }

    #[test]
    fn test_unpack_green() {
        let result = unpackRGB(0x00FF00);

        assert_vec3_close(
            result,
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );
    }

    #[test]
    fn test_unpack_blue() {
        let result = unpackRGB(0x0000FF);

        assert_vec3_close(
            result,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        );
    }

    #[test]
    fn test_unpack_gray() {
        let result = unpackRGB(0x808080);

        assert_vec3_close(
            result,
            Vec3 {
                x: 128.0 / 255.0,
                y: 128.0 / 255.0,
                z: 128.0 / 255.0,
            },
        );
    }

    #[test]
    fn test_unpack_arbitrary_color() {
        let result = unpackRGB(0x336699);

        assert_vec3_close(
            result,
            Vec3 {
                x: 51.0 / 255.0,
                y: 102.0 / 255.0,
                z: 153.0 / 255.0,
            },
        );
    }

    #[test]
    fn test_unpack_channel_boundaries() {
        let result = unpackRGB(0x010203);

        assert_vec3_close(
            result,
            Vec3 {
                x: 1.0 / 255.0,
                y: 2.0 / 255.0,
                z: 3.0 / 255.0,
            },
        );
    }

    #[test]
    fn test_unpack_ignores_upper_bits() {
        let result = unpackRGB(0xFF336699);

        assert_vec3_close(
            result,
            Vec3 {
                x: 51.0 / 255.0,
                y: 102.0 / 255.0,
                z: 153.0 / 255.0,
            },
        );
    }
}