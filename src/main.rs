mod vector;
mod sphere;
mod colour;
mod utils;
mod debug;

use crate::colour::write_color;
use crate::sphere::Sphere;
use crate::vector::{Vec3};
use colour::unpack_rgb;
use utils::*;
use std::{env};
use std::io;



const SHADOWFACTOR: f64 = 0.1;
const CAMERA: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
const DEFAULT_COLOR: u32 = 0x3f3f3f;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    printdebug!("Running in debug mode");
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        dbg!(&args);
        panic!("Expected 3 program args. Recieved {}\n Usage: Cargo run <input_path> <output_path>", args.len());
    }

    // let scene: Scene = create_scene_from_file(args[1]);
    let scene = create_scene_from_file(&args[1]).unwrap();

    printdebug!(format!("{:#?}", scene));

    let mut ppm_file = create_output_ppm_file(&args[2], &scene)?;
    render(&mut ppm_file, scene);
    // write_color(&mut ppm_file, Vec3{ x: 0.5, y: 0.5, z: 0.5} )?;

    Ok(())
}