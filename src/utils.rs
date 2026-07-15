use crate::CAMERA;
use crate::DEFAULT_COLOR;
use crate::colour::unpack_rgb;
use crate::vector::*;
use crate::sphere::*;
use std::fs::File;
use std::io::Write;

use std::ops::Add;
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

pub fn render(output_file: &mut impl Write, scene: Scene) -> Result<(), Box<dyn std::error::Error>> {
    // Safely handle a missing index without crashing the program
    if let Some(&color) = scene.colors().get(scene.bg_colour_index as usize) {
        let bg_colour = unpack_rgb(color);
        // Use your background colour here
    } else {
        // Fallback if the index was invalid or out of bounds
        let bg_colour = unpack_rgb(DEFAULT_COLOR); 
    }


    let bg_colour = unpack_rgb(scene.colors()[scene.bg_colour_index as usize]);
    
    for y in 0..scene.image_height {
        for x in 0..scene.image_width {
            // variable to keep track of pixel colour; divided by 9 before printing
            let mut pixel_colour_average = Vec3{ x: 0.0, y: 0.0, z: 0.0 };
            

            let mut ray = Vec3 {
                x: (x as f64 + 0.5) * scene.viewport.pixel_width.x - scene.viewport.half_width_vec.x,
                y: scene.viewport.half_height_vec.y - (y as f64 + 0.5) * scene.viewport.pixel_height.y,
                z: -(scene.viewport.focal_length as f64),
            };
            ray = ray + CAMERA;     // ray is a vector from camera to pixel location on viewport

            let mut t = f64::MAX;
            let mut min_t = f64::MAX;   // scales the rayDir vector to the intersection point
            let mut closest_sphere_index = 0;

            let closest_hit = scene.spheres
                .iter()
                .enumerate()
                .filter_map(|(i, sphere)| {
                    let mut t = f64::MAX;
                    if sphere.does_intersect(&CAMERA, &ray, &mut t) {
                        Some((t, i))
                    } else {
                        None
                    }
                })
                .min_by(|a,b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

            // check for sphere intersection
            if let Some((min_t, closest_sphere_index)) = closest_hit {
                let intersection_point = CAMERA + (ray * min_t);

                if let Some(sphere) = scene.spheres().get(closest_sphere_index) {
                    let direction = intersection_point - sphere.pos;
                    
                    // 2. Safely normalize to prevent division by zero
                    let normal = direction.normalize();
                }
            }

        }
    }

    Ok(())
}

fn calculate_lighting_and_shadows() {
    unimplemented!()
}