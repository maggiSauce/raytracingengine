mod vector;
mod sphere;
mod colour;

use crate::sphere::Sphere;
use crate::vector::{Vec3};
use colour::unpackRGB;
use core::num;
use std::{env, fs};
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Light {
    position: Vec3,
    brightness: f64,
}

#[derive(Debug)]
struct Viewport {
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,
}

#[derive(Debug)]
struct Scene {
    image_height: i32,
    image_width: i32,

    viewport: Viewport,
    light: Light,

    colors: Vec<u32>,
    bg_colour_index: u32,

    spheres: Vec<Sphere>,
}

const SHADOWFACTOR: f64 = 0.1;
const CAMERA: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

fn create_scene_from_file(path: &str) -> Result<Scene, io::Error>{
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
    let viewport = Viewport {
        viewport_height: viewport_height,
        viewport_width: (width as f32 / height as f32) * viewport_height,
        focal_length: lines.next().unwrap().parse().unwrap(),
    };
    
    // Light
    let mut light_parts = lines.next().unwrap().split_whitespace();
    let light = Light {
        position: Vec3 {
            x: light_parts.next().unwrap().parse().unwrap(),
            y: light_parts.next().unwrap().parse().unwrap(),
            z: light_parts.next().unwrap().parse().unwrap(),
        },
        brightness: light_parts.next().unwrap().parse().unwrap(),
    };

    // colors
    lines.next();   // consume the numcolors value
    let mut colours: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| u32::from_str_radix(s.trim_start_matches("0x"), 16).unwrap())
        .collect();
    colours.sort();
    let bg_colour_index:u32 = lines.next().unwrap().parse().unwrap();

    // Spheres
    let mut spheres = Vec::new();
    let num_spheres: u32 = lines.next().unwrap().parse().unwrap();
    for i in 0..num_spheres {
        let mut sphere_parts = lines.next().unwrap().split_whitespace();

        let mut sphere = Sphere {
            pos: Vec3 {
                x: sphere_parts.next().unwrap().parse().unwrap(),
                y: sphere_parts.next().unwrap().parse().unwrap(),
                z: sphere_parts.next().unwrap().parse().unwrap(),
            },
            r: sphere_parts.next().unwrap().parse().unwrap(),
            colour: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
        };

        let sphere_col_index: usize = sphere_parts.next().unwrap().parse().unwrap();
        sphere.colour = unpackRGB(colours[sphere_col_index]);
        
        spheres.push(sphere);
    }


    let scene = Scene {
        image_height: height,
        image_width: width,
        viewport: viewport,
        light: light,
        colors: colours,
        bg_colour_index: bg_colour_index,
        spheres: spheres,
    };

    Ok(scene)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        dbg!(&args);
        panic!("Expected 3 program args. Recieved {}", args.len());
    }

    // let scene: Scene = create_scene_from_file(args[1]);
    let scene = create_scene_from_file(&args[1]).unwrap();
    println!("{:#?}", scene);
}

