use crate::coordinate::Coordinate;
use crate::objects::*;
use crate::objects::{Hit, Shape};
use macaw::Vec3;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub shapes: Vec<Shape>,
    pub lights: Vec<Light>,
    pub shadow: f64,
}

impl Scene {
    pub fn new() -> Self {
        let width = 800;
        let height = 600;
        let fov = 90f32;
        let irradiance = 0.016;
        let mut shapes: Vec<Shape> = vec![];
        let mut lights: Vec<Light> = vec![];
        let shadow: f64 = 0.1;
        // let light = Light {
        //     direction: Vec3::new(-0.25f32, -1f32, -1f32),
        //     color: Color {
        //         r: 255f64,
        //         g: 255f64,
        //         b: 255f64,
        //     },
        //     strength: 0.004f32,
        // };
        let light = Light::Spherical(SphericalLight {
            position: Coordinate {
                x: -2.0,
                y: 10.0,
                z: -3.0,
            },
            color: Color {
                r: 0.3,
                g: 0.8,
                b: 0.3,
            },
            strength: 24000.0,
        });
        let sphere = Shape::new_sphere(
            Coordinate {
                x: (-3.0),
                y: (0.0),
                z: (-6.0),
            },
            2.0f32,
            Color {
                r: (0.2 * 255.0),
                g: (0.2 * 255.0),
                b: (1.0 * 255.0),
            },
            0.004,
            0.5,
        );

        let sphere2 = Shape::new_sphere(
            Coordinate {
                x: (2.0),
                y: (-1.0),
                z: (-4.0),
            },
            1.0,
            Color {
                r: 1.0 * 255.0,
                g: 0.2 * 255.0,
                b: 0.2 * 255.0,
            },
            0.0038,
            0.6,
        );

        let plane = Shape::new_plane(
            Coordinate {
                x: (0.0),
                y: (-2.0),
                z: (0.0),
            },
            Vec3::new(0.0, -1.0, 0.0),
            Color {
                r: (0.1 * 255.0),
                g: (0.1 * 255.0),
                b: (0.1 * 255.0),
            },
            0.008,
            0.4,
        );
        let plane2 = Shape::new_plane(
            Coordinate {
                x: (0.0),
                y: (0.0),
                z: (-20.0),
            },
            Vec3::new(0.0, 0.0, -1.0),
            Color {
                r: (0.1 * 255.0),
                g: (0.1 * 255.0),
                b: (1.0 * 255.0),
            },
            0.005,
            0.4,
        );

        let plane3 = Shape::new_plane(
            Coordinate {
                x: (-10.0),
                y: (0.0),
                z: (-20.0),
            },
            Vec3::new(-1.0, 0.0, 0.0),
            Color {
                r: (1.0 * 255.0),
                g: (0.0 * 255.0),
                b: (0.2 * 255.0),
            },
            0.008,
            0.4,
        );
        lights.push(light);
        shapes.push(sphere);
        shapes.push(sphere2);
        //shapes.push(sphere3);
        shapes.push(plane);
        shapes.push(plane2);
        shapes.push(plane3);
        Self {
            width,
            height,
            fov,
            shapes,
            lights,
            shadow,
        }
    }

    //https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/
    pub fn trace(&self, ray: &Ray) -> Option<Hit> {
        self.shapes
            .iter()
            .filter_map(|s| s.hit(ray).map(|d| Hit::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
