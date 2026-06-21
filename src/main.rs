use crate::vector::{Vec3, add};
use std::env;
use std::fs::File;
use std::io;

mod vector;

struct Light {
    position: Vec3,
    rgb: Vec3,
    brightness: f64,
}

struct Camera {
    viewport_height: f32,
    viewport_weidth: f32,
    focal_lentgh: f32,
}




struct Scene {
    image_height: i32,
    image_width: i32,

    camera: Camera,
    light: Light,

    colors: Vec<String>,
    background_color: usize,
}

fn create_scene_from_file(file_path: String) -> Scene {
    let file = File::open(file_path)?;
    let mut lines = io::BufReader::new(file).io::lines();



}

const SHADOWFACTOR: f64 = 0.1;
const CAMERA: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        dbg!(&args);
        panic!("Expected 3 program args. Recieved {}", args.len());
    }

    let scene: Scene = create_scene_from_file(args[1]);
    
    let (image_H, image_W): (i32, i32);
    let focal_length: f64;
    let bgColour: Vec3;
}