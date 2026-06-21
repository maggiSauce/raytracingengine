use crate::vector::{Vec3};

pub struct Sphere {
    pub r: f32,
    pub pos: Vec3,
    pub colour: Vec3,
}

impl Sphere {
    fn new(radius: f32, position: Vec3, colour: Vec3) -> Self {
        Self {
            r: radius,
            pos: position,
            colour,
        }
    }
}
