mod vector;
mod sphere;
mod colour;
mod utils;

use crate::sphere::Sphere;
use crate::vector::{Vec3};
use colour::unpack_rgb;
use utils::*;
use std::{env};
use std::io;



const SHADOWFACTOR: f64 = 0.1;
const CAMERA: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        dbg!(&args);
        panic!("Expected 3 program args. Recieved {}", args.len());
    }

    // let scene: Scene = create_scene_from_file(args[1]);
    let scene = create_scene_from_file(&args[1]).unwrap();
    println!("{:#?}", scene);

    let ppm_file = create_output_ppm_file(&args[2], &scene);

    Ok(())
}

