//use crate::objects::{Color, Hit, Ray, Shape};
use crate::coordinate::Coordinate;
use crate::objects::*;
use crate::scene::Scene;
use image::DynamicImage;
use macaw::Vec3;
use std::fs;
use std::io::{BufWriter, Write};

mod coordinate;
mod objects;
mod scene;

fn main() {
    let scene = Scene::new();
    pixelator(&scene);
}

fn pixelator(scene: &Scene) {
    let preamble = format!("P3\n{} {}\n255\n", scene.width, scene.height);

    //set up the File
    let f = fs::File::create("test.ppm").expect("could not create file");
    println!("file created");
    //let f = fs::File::open("test.ppm").expect("could not open file");
    println!("file opened");
    let mut stream = BufWriter::new(f);
    stream.write(preamble.as_bytes());

    let mut colours = String::new();

    for i in (0..scene.height) {
        for j in (0..scene.width) {
            let ray = Ray::first(j, i, scene);
            let hit = scene.trace(&ray);
            let black = Color {
                r: 0f64,
                g: 0f64,
                b: 0f64,
            };
            let color = hit
                .map(|s| rgb(&calculate_pixels(scene, &ray, &s)))
                .unwrap_or(black);

            colours = format!("\n {} {} {}", color.r as u8, color.g as u8, color.b as u8);
            stream.write(colours.as_bytes()).unwrap();
        }
    }
}

fn rgb(color: &Color) -> Color {
    let r = (color.r * 255.0);
    let g = (color.g * 255.0);
    let b = (color.b * 255.0);
    Color { r, g, b }
}

fn calculate_pixels(scene: &Scene, ray: &Ray, hit: &Hit) -> Color {
    let hit_Coordinate = ray.origin + (ray.direction * hit.distance);
    let surface_normal = hit.shape.surface_normal(&hit_Coordinate);

    let mut color = Color {
        r: 0.0,
        b: 0.0,
        g: 0.0,
    };
    for light in &scene.lights {
        let direction_to_light = light.direction_from(&hit_Coordinate);

        let shadow_ray = Ray {
            origin: hit_Coordinate + (surface_normal * scene.shadow as f32),
            direction: direction_to_light,
        };

        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none()
            || shadow_intersection.unwrap().distance > light.distance(&hit_Coordinate) as f32;

        let light_intensity = if in_light {
            light.strength(&hit_Coordinate)
        } else {
            0.0
        };
        let light_power =
            (surface_normal.dot(direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = hit.shape.irradiance() / std::f32::consts::PI;

        let light_color = light.color() * light_power as f64 * light_reflected as f64;
        color = color + (hit.shape.color() * light_color);
    }
    color = color.clamp();

    //for reflections
    let reflection = Ray {
        origin: (surface_normal * scene.shadow as f32) + hit_Coordinate,
        direction: ray.direction - (2f32 * ray.direction.dot(surface_normal) * surface_normal),
    };

    let reflectivity = hit.shape.reflectivity();
    if reflectivity > 0.0 {
        for i in 0..2 {
            let trace_val = scene.trace(&reflection);
            color = (color * hit.shape.reflectivity())
                + reflectivity
                    * trace_val
                        .map(|s| calculate_pixels(scene, &reflection, &s))
                        .unwrap_or(Color {
                            r: 0f64,
                            g: 0f64,
                            b: 0f64,
                        });
        }
    }

    color
}
