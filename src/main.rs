use crate::vector::{Vec3, add};

mod vector;

struct light {
    position: Vec3,
    rgb: Vec3,
    brightness: f64,
}

const SHADOWFACTOR: f64 = 0.1;
const CAMERA: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
fn main() {
    let (image_H, image_W): (i32, i32);
    let focal_length: f64;
    let bgColour: Vec3;
}
