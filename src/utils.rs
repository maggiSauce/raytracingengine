use crate::vector::*;
use crate::sphere::*;
use std::fs::File;
use std::io::Write;

use std::{fs};
use std::io::{self};

#[derive(Debug)]
pub struct Light {
    pub position: Vec3,
    pub brightness: f64,
}

#[derive(Debug)]
pub struct Viewport {
    pub height: f32,
    pub width: f32,
    pub focal_length: f32,

    pub height_vec: Vec3,
    pub half_height_vec: Vec3,

    pub width_vec: Vec3,
    pub half_width_vec: Vec3,

    pub pixel_width: Vec3,
    pub pixel_height: Vec3,
}

#[derive(Debug)]
pub struct Scene {
    pub image_height: i32,
    pub image_width: i32,

    pub viewport: Viewport,
    pub light: Light,

    pub bg_colour_index: u32,
    colors: Vec<u32>,

    spheres: Vec<Sphere>,
}

impl Scene {
    /// returns dimensions of the scene in the format (imageH, imageW)
    pub fn colors(&self) -> &[u32] {
        &self.colors
    }

    pub fn spheres(&self) -> &[Sphere] {
        &self.spheres
    }
}

pub fn create_scene_from_file(path: &str) -> Result<Scene, io::Error>{
    let contents = fs::read_to_string(path)?;
    let mut lines = contents.lines();

    // Dimensions
    let (image_width, image_height) = lines
        .next()
        .unwrap()
        .split_once(" ")
        .unwrap();
    let image_height: i32 = image_height.trim().parse().unwrap();
    let image_width: i32  = image_width.trim().parse().unwrap();

    // Camera
    let viewport_height: f32 = lines.next().unwrap().parse().unwrap();
    let viewport_width: f32 = (image_width as f32 / image_height as f32) * viewport_height;
    
    let viewport_height_vec = Vec3 { x: 0.0, y: image_height as f64, z: 0.0 };
    let viewport_width_vec = Vec3 { x: image_width as f64, y: 0.0, z: 0.0 };

    let viewport = Viewport {
        height: viewport_height,
        width: viewport_width,
        focal_length: lines.next().unwrap().parse().unwrap(),

        height_vec: viewport_height_vec,
        width_vec: viewport_width_vec,

        half_height_vec: viewport_height_vec / 2.0,
        half_width_vec: viewport_width_vec / 2.0,

        pixel_height: viewport_height_vec / image_height as f64,
        pixel_width: viewport_width_vec / image_width as f64,
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
        image_height,
        image_width,
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