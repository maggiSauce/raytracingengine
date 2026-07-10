use crate::vector::*;
use crate::sphere::*;
use std::fs::File;
use std::io::Write;

use std::{fs};
use std::io::{self};

#[derive(Debug)]
pub struct Light {
    position: Vec3,
    brightness: f64,
}

#[derive(Debug)]
pub struct Viewport {
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,
}

#[derive(Debug)]
pub struct Scene {
    image_height: i32,
    image_width: i32,

    viewport: Viewport,
    light: Light,

    colors: Vec<u32>,
    bg_colour_index: u32,

    spheres: Vec<Sphere>,
}

pub fn create_scene_from_file(path: &str) -> Result<Scene, io::Error>{
    let contents = fs::read_to_string(path)?;
    let mut lines = contents.lines();

    // Dimensions
    let (width, height) = lines
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
    for _i in 0..num_spheres {
        let sphere_parts = lines.next().unwrap().split_whitespace();
        spheres.push(create_sphere(sphere_parts, &colours));
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

pub fn create_output_ppm_file(path: &str, scene: &Scene) -> io::Result<File> {
    let mut ppm_file = File::create(path)?;
    writeln!(ppm_file, "P3")?;
    writeln!(ppm_file, "{} {}", scene.image_width, scene.image_height)?;
    writeln!(ppm_file, "255")?;

    Ok(ppm_file)
}