use crate::vector::{Vec3};
use std::{env, fs};
use std::io::{self, BufRead, BufReader};

mod vector;
mod sphere;

#[derive(Debug)]
struct Light {
    position: Vec3,
    brightness: f64,
}

struct Camera {
    viewport_height: f32,
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

fn create_scene_from_file(path: &str) -> Result<(), io::Error>{
    let contents = fs::read_to_string(path)?;
    let mut lines = contents.lines();

    // Dimensions
    let (height, width) = lines
        .next()
        .unwrap()
        .split_once(" ")
        .unwrap();
    let height: i32 = height.trim().parse().unwrap();
    let width: i32  = width.trim().parse().unwrap();

    // Camera
    let viewport_height: f32 = lines.next().unwrap().parse().unwrap();
    let focal_length: f32 = lines.next().unwrap().parse().unwrap();

    // Light
    let mut light_parts = lines.next().unwrap().split_whitespace();
    let light = Light {
        position: Vec3 {
            x: light_parts.next().unwrap().parse().unwrap(),
            y: light_parts.next().unwrap().parse().unwrap(),
            z: light_parts.next().unwrap().parse().unwrap(),
        },
        brightness: lines.next().unwrap().parse().unwrap(),
    };

    println!("{:?}", light);

    Ok(())
}

const SHADOWFACTOR: f64 = 0.1;
const CAMERA: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        dbg!(&args);
        panic!("Expected 3 program args. Recieved {}", args.len());
    }

    // let scene: Scene = create_scene_from_file(args[1]);
    create_scene_from_file(&args[1]);
}

